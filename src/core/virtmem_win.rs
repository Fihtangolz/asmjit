// Old name: VirtMem_accessToWinProtectFlags
#[cfg(target_family = "windows")]
pub impl Into<i32> for Flags {
    fn into(self) -> i32 {
        use winapi::um::winnt as wi;

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

#[cfg(target_family = "windows")] 
pub fn protect(ptr: NonNull<u8>, size: usize, flag: Flags) -> AsmJitError {
    protection //TODO
    let old_flags: DWORD;

    if winapi::um::memoryapi::VirtualProtect(ptr, size, protection, &old_flags) == 0 {
        None
    }

    Some(Error::InvalidArgument)
}

#[cfg(target_family = "windows")]
pub fn release(ptr: NonNull<u8>, size: usize) -> AsmJitError {
    if !winapi::um::memoryapi::VirtualFree(ptr, 0, winapi::um::memoryapi::MEM_RELEASE) {
        Some(Error::InvalidArgument)
    }

    None
}