use std::ptr::NonNull;
use std::result::Result;
use std::ptr;

use super::globals::{
    Error
};

use super::virtmem::*;
use libc;

pub fn release_dual_mapping(dm: DualMapping, size: usize) -> Option<Error> {
    let mut err = release(dm.ro, size);
    if dm.ro != dm.rw {
        err |= release(dm.rw, size);
    }
    if err {
        return Some(Error::InvalidArgument);
    }

    None
}

pub fn alloc_dual_mapping(size: usize, flags: Flags) -> Result<DualMapping, Error> {
    if size == 0 {
        Err(Error::InvalidArgument);
    }
    if (size as libc::off_t) < 0 {
        Err(Error::TooLarge);
    }
    if flags.contains(Flags::MappingPreferTmp) {
        //TODO: VirtMem_getShmStrategy
    }

    //TODO: VirtMem_openAnonymousMemory

    let mut ptr: [*mut libc::c_void; 2];
    for i in 0..2 {
        ptr[i] = libc::mmap(ptr::null(), size, , libc::MAP_SHARED, , 0);
        if ptr[i] == libc::MAP_FAILED {
            let e1 = libc::errno;
            if i == 1 {
                libc::munmap(ptr[0], size);
                let e2 = libc::errno;
            }
            return;
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
        let protection;
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

pub fn protect(ptr: NonNull<u8>, size: usize, flag: Flags) -> Option<Error> {
    let f = flag.into();
    let cp = ptr.as_ptr() as *mut libc::c_void;
    if libc::mprotect(cp, size, f) != 0 {
        return Some(Error::InvalidArgument);
    }
    None
}

pub fn release(ptr: NonNull<u8>, size: usize) -> Option<Error> {
    let cp = ptr.as_ptr() as *mut libc::c_void;
    if libc::munmap(cp, size) != 0 {
        return Some(Error::InvalidArgument);
    }

    None
}

pub fn alloc(size: usize, flags: Flags) -> Result<*mut u8, Error> {
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

