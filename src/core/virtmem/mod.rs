#[cfg(target_family = "windows")]
mod win;
#[cfg(target_family = "windows")]
use win as target_impl;

#[cfg(target_family = "unix")] 
mod unix;
#[cfg(target_family = "unix")] 
use unix as target_impl;

use std::sync::Once;
use crate::core::globals::Error;

bitflags! {
    /// Virtual memory and memory mapping flags.
    pub struct Flags: u32 {
        /// No access flags.
        const ACCESS_NONE = 0x00000000;
        /// Memory is readable.
        const ACCESS_READ = 0x00000001;
        /// Memory is writable (implies read access).
        const ACCESS_WRITE = 0x00000002;
        /// Memory is executable (implies read access).
        const ACCESS_EXECUTE = 0x00000004;
    
        /// A combination of `kAccessRead | kAccessWrite`
        const ACCESS_READ_WRITE = 0x00000003;
    
        /// Not an access flag, only used by `allocDualMapping()` to override the
        /// default allocation strategy to always use a temporary directory instead
        /// on "/dev/shm" (on POSIX systems). Please note that this flag will be
        /// ignored if the operating system allows to allocate an executable memory
        /// by a different API than `open()` or `shm_open()`. For example on Linux
        /// `memfd_create()` is preferred and on BSDs `shm_open(SHM_ANON, ...)` is
        /// used if SHM_ANON is defined.
        const MAPPING_PREFER_TMP = 0x80000000;
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
            INFO = target_impl::info();
        });
        &INFO
    }
}

/// Allocates virtual memory by either using `VirtualAlloc()` (Windows)
/// or `mmap()` (POSIX).
///
/// \note `size` should be aligned to a page size, use \ref VirtMem::info()
/// to obtain it. Invalid size will not be corrected by the implementation
/// and the allocation would not succeed in such case.
pub unsafe fn alloc(size: usize, flags: Flags) -> Result<*mut u8, Error> {
    target_impl::alloc(size, flags)
}

/// Releases virtual memory previously allocated by \ref  VirtMem::alloc() or
/// \ref VirtMem::allocDualMapping().
///
/// \note The size must be the same as used by \ref VirtMem::alloc(). If the
/// size is not the same value the call will fail on any POSIX system, but
/// pass on Windows, because of the difference of the implementation.
pub unsafe fn release(ptr: *mut u8, size: usize) -> Option<Error> {
    target_impl::release(ptr, size)
}

/// A cross-platform wrapper around `mprotect()` (POSIX) and `VirtualProtect`
/// (Windows).
pub unsafe fn protect(ptr: *mut u8, size: usize, flag: Flags) -> Option<Error> {
    target_impl::protect(ptr, size, flag)
}

impl DualMapping {
    /// Allocates virtual memory and creates two views of it where the first view
    /// has no write access. This is an addition to the API that should be used
    /// in cases in which the operating system either enforces W^X security policy
    /// or the application wants to use this policy by default to improve security
    /// and prevent an accidental (or purposed) self-modifying code.
    ///
    /// The memory returned in the `dm` are two independent mappings of the same
    /// shared memory region. You must use \ref VirtMem::releaseDualMapping() to
    /// release it when it's no longer needed. Never use `VirtMem::release()` to
    /// release the memory returned by `allocDualMapping()` as that would fail on
    /// Windows.
    pub unsafe fn alloc(size: usize, flags: Flags) -> Result<DualMapping, Error> {
        target_impl::alloc_dual_mapping(size, flags)
    }

    /// Releases the virtual memory mapping previously allocated by
    /// [alloc_dual_mapping](alloc_dual_mapping).
    pub unsafe fn release(self, size: usize) -> Option<Error> {
        target_impl::release_dual_mapping(self, size)
    }
}


