use std::sync::Once;

#[cfg(target_family = "windows")]
mod win;
#[cfg(target_family = "windows")]
use win::*;

#[cfg(target_family = "unix")] 
mod unix;
#[cfg(target_family = "unix")] 
use unix::*;

bitflags! {
    /// Virtual memory and memory mapping flags.
    pub struct Flags: u32 {
        /// No access flags.
        const AccessNone = 0x00000000;
        /// Memory is readable.
        const AccessRead = 0x00000001;
        /// Memory is writable (implies read access).
        const AccessWrite = 0x00000002;
        /// Memory is executable (implies read access).
        const AccessExecute = 0x00000004;
    
        /// A combination of `kAccessRead | kAccessWrite`
        const AccessReadWrite = 0x00000003;
    
        /// Not an access flag, only used by `allocDualMapping()` to override the
        /// default allocation strategy to always use a temporary directory instead
        /// on "/dev/shm" (on POSIX systems). Please note that this flag will be
        /// ignored if the operating system allows to allocate an executable memory
        /// by a different API than `open()` or `shm_open()`. For example on Linux
        /// `memfd_create()` is preferred and on BSDs `shm_open(SHM_ANON, ...)` is
        /// used if SHM_ANON is defined.
        const MappingPreferTmp = 0x80000000;
    }
}

// Virtual memory information.
pub struct Info {
    /// Page size is the size of each memory "page" in the memory paging model of virtual memory.  
    /// It's the size of the span of addresses having the same address translation (between virtual addresses and physical addresses).  
    /// Although x86 CPUs support 2 MiB pages and 4 MiB pages, and x64 CPUs support 1 GiB pages (and multiple sizes ranging from 4 kiB through 256 MiB)
    pub page_size: u32,

    /// The granularity with which virtual memory is allocated.
    /// That is, the value to which alignment will occur up - minimal allocation unit.
    /// For example, we request to allocate `1+page_granularity` bytes, so system will reserve an address space of `2*page_granularity` bytes.
    ///
    /// It also mean pointer's address is always bound of it, that is addres is a multiple of `page_granularity`.
    /// That property is extremely convenient for finding a pool header. You can just align current address to `page_granularity`.
    ///
    /// NOTE: For x86/AMD64 platform this value was hard coded as [64 KiB in the past](https://devblogs.microsoft.com/oldnewthing/20031008-00/?p=42223), 
    /// but other hardware architectures may require different values.
    pub page_granularity: u32,
}

/// Dual memory mapping used to map an anonymous memory into two memory regions
/// where one region is read-only, but executable, and the second region is
/// read+write, but not executable. Please see \ref VirtMem::allocDualMapping()
/// for more details.
pub struct DualMapping {
    // Pointer to data with 'Read' or 'Read+Execute' access.
    pub ro: *mut u8,
    // Pointer to data with 'Read-Write' access, but never 'Write+Execute'.
    pub rw: *mut u8,
}

/// Returns [virtual memory information](Info).
pub fn info() -> &'static Info {
    static mut INFO: Info = Info{page_size: 0, page_granularity: 0}; 
    static INIT: Once = Once::new();
    unsafe {
        INIT.call_once(|| {
            INFO = get_info();
        });
        &INFO
    }
}

pub enum ShmStrategy {
    Unknown,
    DevShm,
    TmpDir,
}

