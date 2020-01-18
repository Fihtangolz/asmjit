// Virtual memory and memory mapping flags.
bitflags! {
    pub struct Flags: u32 {
        // No access flags.
        const AccessNone = 0x00000000;
        // Memory is readable.
        const AccessRead = 0x00000001;
        // Memory is writable (implies read access).
        const AccessWrite = 0x00000002;
        // Memory is executable (implies read access).
        const AccessExecute = 0x00000004;
    
        // A combination of `kAccessRead | kAccessWrite`
        const AccessReadWrite = 0x00000003;
    
        // Not an access flag, only used by `allocDualMapping()` to override the
        // default allocation strategy to always use a temporary directory instead
        // on "/dev/shm" (on POSIX systems). Please note that this flag will be
        // ignored if the operating system allows to allocate an executable memory
        // by a different API than `open()` or `shm_open()`. For example on Linux
        // `memfd_create()` is preferred and on BSDs `shm_open(SHM_ANON, ...)` is
        // used if SHM_ANON is defined.
        const MappingPreferTmp = 0x80000000;
    }
}

pub struct Info {
    pub page_size: u32,
    pub page_granularity: u32,
}

pub fn info() -> Info {
    unimplemented!();
}

pub struct DualMapping {
    ro: *mut u8,
    rw: *mut u8,
}
pub enum ShmStrategy {
    Unknown,
    DevShm,
    TmpDir,
}

use super::virtmem_unix;
