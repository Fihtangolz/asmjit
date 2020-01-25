use std::sync::Mutex;
use super::virtmem;
use super::zonetree::ZoneTree;
use std::alloc::alloc;
use std::ptr::null_mut;

/// Parameters that can be passed to `JitAllocator` constructor.
///
/// Use it like this:
///
/// ```
/// // Zero initialize (zero means the default value) and change what you need.
/// JitAllocator::CreateParams params {};
/// params.blockSize = 1024 * 1024;
///
/// // Create the allocator.
/// JitAllocator allocator(&params);
/// ```
struct CreateParams {
    /// Allocator options, see \ref JitAllocator::Options.
    ///
    /// No options are used by default.
    options: Options,

    /// Base size of a single block in bytes (default 64kB).
    ///
    /// \remarks Block size must be equal or greater to page size and must be
    /// power of 2. If the input is not valid then the default block size will
    /// be used instead.
    block_size: u32,

    /// Base granularity (and also natural alignment) of allocations in bytes
    /// (default 64).
    ///
    /// Since the `JitAllocator` uses bit-arrays to mark used memory the
    /// granularity also specifies how many bytes correspond to a single bit in
    /// such bit-array. Higher granularity means more waste of virtual memory
    /// (as it increases the natural alignment), but smaller bit-arrays as less
    /// bits would be required per a single block.
    granularity: u32,

    /// Patter to use to fill unused memory.
    ///
    /// Only used if \ref kOptionCustomFillPattern is set.
    fill_pattern: u32,
}

bitflags! {
    pub struct Options: u32 {
        // Enables the use of an anonymous memory-mapped memory that is mapped into
        // two buffers having a different pointer. The first buffer has read and
        // execute permissions and the second buffer has read+write permissions.
        //
        // See \ref VirtMem::allocDualMapping() for more details about this feature.
        const USE_DUAL_MAPPING = 0x00000001;

        // Enables the use of multiple pools with increasing granularity instead of
        // a single pool. This flag would enable 3 internal pools in total having
        // 64, 128, and 256 bytes granularity.
        //
        // This feature is only recommended for users that generate a lot of code
        // and would like to minimize the overhead of `JitAllocator` itself by
        // having blocks of different allocation granularities. Using this feature
        // only for few allocations won't pay off as the allocator may need to
        // create more blocks initially before it can take the advantage of
        // variable block granularity.
        const Use_Multiple_Pools = 0x00000010;

        // Always fill reserved memory by a fill-pattern.
        //
        // Causes a new block to be cleared by the fill pattern and freshly
        // released memory to be cleared before making it ready for another use.
        const FILL_UnusedMemory = 0x00000100;

        // When this flag is set the allocator would immediately release unused
        // blocks during `release()` or `reset()`. When this flag is not set the
        // allocator would keep one empty block in each pool to prevent excessive
        // virtual memory allocations and deallocations in border cases, which
        // involve constantly allocating and deallocating a single block caused
        // by repetitive calling `alloc()` and `release()` when the allocator has
        // either no blocks or have all blocks fully occupied.
        const ImmediateRelease = 0x00001000;

        // Use a custom fill pattern, must be combined with `kFlagFillUnusedMemory`.
        const CustomFillPattern = 0x00010000;
    }
}

/// Statistics about `JitAllocator`.
struct Statistics {
    /// Number of blocks `JitAllocator` maintains.
    block_count: usize,
    /// How many bytes are currently used / allocated.
    used_size: usize,
    /// How many bytes are currently reserved by the allocator.
    reserved_size: usize,
    /// Allocation overhead (in bytes) required to maintain all blocks.
    overhead_size: usize,
}

impl Statistics {
    pub fn new() -> Self {
        Self {
            block_count: 0,
            used_size: 0,
            reserved_size: 0,
            overhead_size: 0,
        }
    }
  
    /// Returns count of blocks managed by `JitAllocator` at the moment.
    pub fn block_count(&self) -> usize { 
        self.block_count 
    }
  
    /// Returns how many bytes are currently used.
    pub fn used_size(&self) -> usize { 
        self.used_size 
    }
    /// Returns the number of bytes unused by the allocator at the moment.
    pub fn unused_size(&self) -> usize { 
        self.reserved_size - self.used_size 
    }
    /// Returns the total number of bytes bytes reserved by the allocator (sum of sizes of all blocks).
    pub fn reserved_size(&self) -> usize { 
        self.reserved_size
    }
    /// Returns the number of bytes the allocator needs to manage the allocated memory.
    pub fn overhead_size(&self) -> usize { 
        self.overhead_size
    }
  
    pub fn used_size_as_percent(&self) -> f64 {
        let fus = self.used_size as f64;
        let frs = self.reserved_size as f64;
        (fus*100.0)/frs
    }
  
    pub fn unused_size_as_percent(&self) -> f64 {
        let funs = self.unused_size() as f64;
        let frs = self.reserved_size as f64;
        (funs*100.0)/frs
    }
  
    pub fn overhead_size_as_percent(&self) -> f64 {
        let fos = self.overhead_size as f64;
        let frs = self.reserved_size as f64;
        (fos*100.0)/frs
    }
}

// A simple implementation of memory manager that uses `asmjit::VirtMem`
// functions to manage virtual memory for JIT compiled code.
//
// Implementation notes:
//
// - Granularity of allocated blocks is different than granularity for a typical
//   C malloc. In addition, the allocator can use several memory pools having a
//   different granularity to minimize the maintenance overhead. Multiple pools
//   feature requires `kFlagUseMultiplePools` flag to be set.
//
// - The allocator doesn't store any information in executable memory, instead,
//   the implementation uses two bit-vectors to manage allocated memory of each
//   allocator-block. The first bit-vector called 'used' is used to track used
//   memory (where each bit represents memory size defined by granularity) and
//   the second bit vector called 'stop' is used as a sentinel to mark where
//   the allocated area ends.
//
// - Internally, the allocator also uses RB tree to keep track of all blocks
//   across all pools. Each inserted block is added to the tree so it can be
//   matched fast during `release()` and `shrink()`.
pub struct JitAllocator {
    // Allocator options, see \ref JitAllocator::Options.
    options: Options,
    // Base block size (0 if the allocator is not initialized).
    block_size: u32,
    // Base granularity (0 if the allocator is not initialized).
    granularity: u32,
    // A pattern that is used to fill unused memory if secure mode is enabled.
    fill_pattern: u32,

    /// Lock for thread safety.
    lock: Mutex<()>,
    /// System page size (also a minimum block size).
    page_size: u32,

    /// Blocks from all pools in RBTree.
    tree: ZoneTree<JitAllocatorBlock>,
    /// Allocator pools.
    pools: *mut JitAllocatorPool,
    /// Number of allocator pools.
    pool_count: usize
}

impl JitAllocator {
    /// \name Construction & Destruction
    /// \{
    
    /// Creates a `JitAllocator` instance.
    pub fn new(create_params: &Option<CreateParams>) -> Self {
        let vm_info = virtmem::info();
        let mut cp = create_params.unwrap_or(CreateParams{
            options: Options::empty(),
            block_size: 0,
            granularity: BaseGranularity,
            //X86 and X86_64 - 4x 'int3' instruction.
            fill_pattern: 0xCCCCCCCC, //FIX: so we should handle another platfor 
        });

        // Setup pool count to [1..3].
        let pool_count = if cp.options.contains(Options::Use_Multiple_Pools) {
            1
        } else {
            MultiPoolCount
        };
        
        // Setup block size [64kB..256MB].
        if cp.block_size < 64 * 1024 || cp.block_size > 256 * 1024 * 1024 || cp.block_size.is_power_of_two() {
            cp.block_size = vm_info.page_granularity;
        }

        // Setup granularity [64..256].
        if cp.granularity < 64 || cp.granularity > 256 || cp.granularity.is_power_of_two() {
            cp.granularity = BaseGranularity;
        }
        
        //size_t size = sizeof(JitAllocatorPrivateImpl) + sizeof(JitAllocatorPool) * poolCount;
        let ptr = alloc();
              
        //     JitAllocatorPool* pools = reinterpret_cast<JitAllocatorPool*>((uint8_t*)p + sizeof(JitAllocatorPrivateImpl));
        //     JitAllocatorPrivateImpl* impl = new(p) JitAllocatorPrivateImpl(pools, poolCount);
        
        Self {
            options: cp.options,
            block_size: cp.block_size,
            granularity: cp.granularity,
            fill_pattern: cp.fill_pattern,
            page_size: vm_info.page_size,
        }

        //     for (size_t poolId = 0; poolId < poolCount; poolId++)
        //       new(&pools[poolId]) JitAllocatorPool(granularity << poolId);

    }
    /// Destroys the `JitAllocator` instance and release all blocks held.
    pub fn drop() {unimplemented!(); }
    
    pub fn isInitialized() { 
        unimplemented!(); 
    }
    
    /// Free all allocated memory - makes all pointers returned by `alloc()` invalid.
    ///
    /// \remarks This function is not thread-safe as it's designed to be used when
    /// nobody else is using allocator. The reason is that there is no point of
    /// calling `reset()` when the allocator is still in use.
    pub fn reset() { 
        unimplemented!(); 
    }
    
    /// \}
    
    /// \name Accessors
    /// \{
    
    /// Returns allocator options, see `Flags`.
    pub fn options(&self) -> Options {
        self.options
    }
    /// Tests whether the allocator has the given `option` set.
    pub fn has_option(&self, options: Options) -> bool { 
        self.options.contains(options)
    }
    
    /// Returns a base block size (a minimum size of block that the allocator would allocate).
    pub fn block_size(&self) -> u32 { 
        self.block_size 
    }
    /// Returns granularity of the allocator.
    pub fn granularity(&self) -> u32 { 
        self.granularity
    }
    /// Returns pattern that is used to fill unused memory if `kFlagUseFillPattern` is set.
    pub fn fill_pattern(&self) -> u32 { 
        self.fill_pattern 
    }
    
    /// \}
    
    // \name Alloc & Release
    // \{
    
    // Allocate `size` bytes of virtual memory.
    //
    // \remarks This function is thread-safe.
    pub fn alloc() {
        unimplemented!();
    }
    
    // Release a memory returned by `alloc()`.
    //
    // \remarks This function is thread-safe.
    pub fn release() {
        unimplemented!();
    }
    
    // Free extra memory allocated with `p` by restricting it to `newSize` size.
    //
    // \remarks This function is thread-safe.
    pub fn shrink() {
        unimplemented!();
    }
    
    // \}
    
    // \name Statistics
    // \{
    
    // Returns JIT allocator statistics.
    //
    // \remarks This function is thread-safe.
    pub fn statistics(&self) -> Statistics { 
        let mut stat = Statistics::new();
        self.lock.lock() //FIX: not work 
        
        self.pool_count;
        
        stat.block_count += self.block_count;
        stat.reserved_size += self.reserved_size;
        stat.used_size += self.used_size;
        stat.overhead_size += self.overhead_size
    }
}


struct JitAllocatorPool {
    // Double linked list of blocks.
    blocks: ZoneList<JitAllocatorBlock>,
    // Where to start looking first.
    cursor: *mut JitAllocatorBlock,

    // Count of blocks.
    block_count: u32,
    // Allocation granularity.
    granularity: u16, 
    // Log2(granularity).
    granularity_log2: u8,
    // Count of empty blocks (either 0 or 1 as we won't keep more blocks empty).
    empty_block_count: u8,

    // Number of bits reserved across all blocks.
    total_area_size: usize,
    // Number of bits used across all blocks.
    total_area_used: usize,
    // Overhead of all blocks (in bytes).
    total_overhead_bytes: usize,
}

impl JitAllocatorPool {
    pub fn new(granularity: u32) -> Self {
        Self {
            blocks: ZoneList::new(),
            cursor: unsafe { null_mut() },
            block_count: 0,
            granularity: granularity as u16,
            granularity_log2: ,
            empty_block_count: 0,
            total_area_size: 0,
            total_area_used: 0,
            total_overhead_bytes: 0,
        }
    }

    pub fn reset(&mut self) {
        self.blocks.reset();
        self.cursor = unsafe { null_mut() };
        self.block_count = 0;
        self.total_area_size = 0;
        self.total_area_used = 0;
        self.total_overhead_bytes = 0;
    }

    pub fn bytesize_from_area_size(&self, area_size: u32) -> usize {
        ((self.granularity as u32)*area_size) as usize
    }
    pub fn area_size_from_bytesize() {

    }

    pub fn bitword_count_from_areasize() {

    }
}

struct JitAllocatorBlock{}

// Number of pools to use when `JitAllocator::kOptionUseMultiplePools` is set.
//
// Each pool increases granularity twice to make memory management more
// efficient. Ideal number of pools appears to be 3 to 4 as it distributes
// small and large functions properly.
const MultiPoolCount: usize = 3;

// Minimum granularity (and the default granularity for pool #0).
const BaseGranularity: u32 = 64;

// Maximum block size (16MB).
const MaxBlockSize: usize = 1024 * 1024 * 16;
  

//CPP: chages 
//1) Remove reset from CreateParams
//3) Rename reset into new from Statistics
//4) Remove impl idiom 