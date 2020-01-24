use winapi::um::{
    memoryapi as wm,
    handleapi as wh,
    winnt as wi,
    errhandlingapi as we,
    winbase as wb,
    sysinfoapi as wif,
};
use winapi::shared::minwindef as wd;
use std::ptr;
use super::virtmem::*;

pub fn get_info() -> Info {
    let sys_info: wif:LPSYSTEM_INFO;
    wif::GetSystemInfo(sys_info);

    Info {
        page_size: sys_info.dwPageSize,
        page_granularity: sys_info.dwAllocationGranularity,
    }
}

// Old name: VirtMem_accessToWinProtectFlags
pub impl Into<i32> for Flags {
    fn into(self) -> i32 {
        let protection;
        if self.contains(Flags::AccessExecute) {
            protection = if self.contains(Flags::AccessWrite) {
                wi:PAGE_EXECUTE_READWRITE
            } else {
                wi::PAGE_EXECUTE_READ
            }
        } else if self.contains(Flags::AccessReadWrite) {
            protection = if self.contains(Flags::AccessWrite) {
                wi::PAGE_READWRITE
            } else {
                wi::PAGE_READONLY 
            }
        } else {
            protection = wi::PAGE_NOACCESS;
        }
        protection
    }
}

pub fn access_to_win_desired_access(flag: Flags) -> wd::DWORD {
    let access = if flags.contains(Flags::AccessWrite) {
        wm::FILE_MAP_WRITE
    } else {
        wm::FILE_MAP_READ
    }
    if flag.contains(Flags::AccessExecute) {
        access |= wm::FILE_MAP_EXECUTE;
    }
    access
}

pub fn release(ptr: NonNull<u8>, size: usize) -> Option<Error> {
    if !wm::VirtualFree(ptr, 0, wi::MEM_RELEASE) {
        //TODO: VirtualFree failed
    }

    None
}

pub fn protect(ptr: NonNull<u8>, size: usize, flag: Flags) -> Option<Error> {
    let old_flags: DWORD;
    if wm::VirtualProtect(ptr, size, flag.into(), &old_flags) == 0 {
        None
    }

    //TODO: VirtualProtect failed
}

pub unsafe fn alloc(size: usize, flag: Flags) -> Result<*mut u8, Error> {
    if size == 0 {
        panic!("size cannot be zero");
    }

    let res = wm::VirtualAlloc(ptr::null(), size, wi::MEM_COMMIT | wi::MEM_RESERVE, flag.into());
    if !res {
        //TODO: VirtualAlloc failed
    }

}

pub unsafe fn alloc_dual_mapping(size: usize, flags: Flags) -> Result<DualMapping, Error> {
    if size == 0 {
        panic!("size cannot be zero");
    }
    
    let fh: wi::HANDLE = ptr::null_mut();
    defer!{{ 
        if fh != ptr::null() {
            wh::CloseHandle(fh);
            //TODO: CloseHandle failed
        }
    }};

    fh = wm::CreateFileMappingW(
        wh::INVALID_HANDLE_VALUE,
        ptr::null(),
        wi::PAGE_EXECUTE_READWRITE,
        wd::DWORD,
        wd::DWORD,
        ptr::null(),
    );

    if fn == ptr::null() {
        //TODO: CreateFileMappingW failed
    }

    let mut ptr: [*mut wd::LPVOID; 2] = MaybeUninit::uninit().assume_init();
    for i in 0..2 {
        desired_access = access_to_win_desired_access(flgas);
        ptr[i] = wm::MapViewOfFile(fh, desired_access, 0, 0, size);
         //TODO: MapViewOfFile failed
    }

    Ok(DualMapping{
        ro: ptr[0] as *mut u8,
        rw: ptr[1] as *mut u8,
    })
}

pub unsafe fn release_dual_mapping(dm: DualMapping, size: usize) -> Option<Error> {
    wm::UnmapViewOfFile(dm->ro);
     //TODO: clarify can MapViewOfFile provide equal pointers. It seems impossible
     //TODO: UnmapViewOfFile failed
    if dm.ro != dm.rw { 
        wm::UnmapViewOfFile(dm->rw);
    }
    //TODO: UnmapViewOfFile failed
}