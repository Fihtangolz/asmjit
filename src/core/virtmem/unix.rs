use std::ptr::NonNull;
use std::result::Result;
use std::ptr;
use std::mem::MaybeUninit;
use std::cmp::max;

use crate::core::globals::{
    Error
};

use crate::core::virtmem::*;
use libc;

const dualMappingFilter: [Flags; 2] = [
    Flags::AccessWrite,
    Flags::AccessExecute,
];

pub fn get_info() -> Info { 
    let page_size = unsafe { libc::sysconf(libc::_SC_PAGESIZE) } as u32;
    Info {
        page_size: page_size,
        page_granularity: max(page_size, 65536),
    }
}

pub unsafe fn release_dual_mapping(dm: DualMapping, size: usize) -> Option<Error> {
    let mut err = release(dm.ro, size);
    //TODO: clarify can mmap provide equal pointers. It seems impossible
    if dm.ro != dm.rw { 
        let mut err2 = release(dm.rw, size);
    }
    // TODO: we should have opportunity unioning multiple error as single
    // if err {
    //     return Some(Error::InvalidArgument);
    // }

    None
}

pub unsafe fn alloc_dual_mapping(size: usize, flags: Flags) -> Result<DualMapping, Error> {
    if size == 0 {
        return Err(Error::InvalidArgument);
    }
    if (size as libc::off_t) < 0 {
        return Err(Error::TooLarge);
    }
    if flags.contains(Flags::MappingPreferTmp) {
        //TODO: VirtMem_getShmStrategy
    }

    //TODO: VirtMem_openAnonymousMemory
    let mut fd: libc::c_int = -1; 
    defer!{{ 
        if fd != -1 {
            libc::close(fd);
        }
    }};
    
    let mut ptr: [*mut libc::c_void; 2] = MaybeUninit::uninit().assume_init();
    for i in 0..2 {
        ptr[i] = libc::mmap(ptr::null_mut(), size, flags.into(), libc::MAP_SHARED, fd, 0);
        if ptr[i] == libc::MAP_FAILED {
            let e1 = libc::__errno_location();
            if i == 1 {
                libc::munmap(ptr[0], size);
                let e2 = libc::__errno_location();
            }
            //TODO
        }
    }

    Ok(DualMapping{
        ro: ptr[0] as *mut u8,
        rw: ptr[1] as *mut u8,
    })
}

// Old name: VirtMem_accessToPosixProtection
impl Into<i32> for Flags {
    fn into(self) -> i32 {
        let mut protection = 0;
        if self.contains(Flags::AccessRead) {
            protection = libc::PROT_READ;
        }
        if self.contains(Flags::AccessWrite) {
            protection = libc::PROT_READ | libc::PROT_WRITE;
        }
        if self.contains(Flags::AccessExecute) {
            protection = libc::PROT_READ | libc::PROT_EXEC;
        }
        protection
    }
}

pub unsafe fn protect(ptr: *mut u8, size: usize, flag: Flags) -> Option<Error> {
    let f = flag.into();
    let cp = ptr as *mut libc::c_void;
    if libc::mprotect(cp, size, f) != 0 {
        return Some(Error::InvalidArgument);
    }
    None
}

pub unsafe fn release(ptr: *mut u8, size: usize) -> Option<Error> {
    let cp = ptr as *mut libc::c_void;
    if libc::munmap(cp, size) != 0 {
        return Some(Error::InvalidArgument);
    }

    None
}

pub unsafe fn alloc(size: usize, flags: Flags) -> Result<*mut u8, Error> {
    if size == 0 {
        return Err(Error::InvalidArgument);
    }
    let protection = flags.into();
    let cp = ptr::null::<u8>() as *mut libc::c_void;
    let ptr = libc::mmap(cp, size, protection, libc::MAP_PRIVATE | libc::MAP_ANONYMOUS, -1, 0);
    if ptr == libc::MAP_FAILED {
        return Err(Error::OutOfMemory);
    }

    Ok(ptr as *mut u8)
}



//VirtMem_openAnonymousMemory
unsafe fn open_anonymous_memory() -> Option<Error> {
    let fd: libc::c_long;

    #[cfg(target_os = "linux")] {
        static mut VM_MEMFD_CREATE_NOT_SUPPORTED: bool = false;
        if !VM_MEMFD_CREATE_NOT_SUPPORTED {
            fd = libc::syscall(libc::SYS_memfd_create, "vmem", 0);
            if fd >= 0 {
                return None;
            }

            let e = libc::__errno_location();
            if e as libc::c_int  == libc::ENOSYS {
                VM_MEMFD_CREATE_NOT_SUPPORTED = true; 
            } else {
                return None; //TODO: add error handling 
            }
        }
    }

    #[cfg(target_os = "freebsd")] {
        fd = libc::shm_open(libc::SHM_ANON, libc::O_RDWR | libc::O_CREAT | libc::O_EXCL, libc::S_IRUSR | libc::S_IWUSR);
        if fd >= 0 {
            
        }
    }

    

    None
}