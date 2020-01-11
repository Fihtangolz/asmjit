enum CallConvVariants {
    /// `__cdecl` calling convention (used by C runtime and libraries).
    X86CDecl,
    /// `__stdcall` calling convention (used mostly by WinAPI).
    X86StdCall,
    /// `__stdcall` calling convention (used mostly by WinAPI).
    X86MsThisCall,
    /// `__fastcall` convention (MSVC/Intel).
    X86MsFastCall,
    /// `__fastcall` convention (GCC and Clang).
    X86GccFastCall,
    /// `regparm(1)` convention (GCC and Clang).
    X86GccRegParm1,
    /// `regparm(2)` convention (GCC and Clang).
    X86GccRegParm2,
    /// `regparm(3)` convention (GCC and Clang).
    X86GccRegParm3, 

    X86LightCall2,
    X86LightCall3,
    X86LightCall4,

    /// X64 calling convention - WIN64-ABI.
    X86Win64,
    /// X64 calling convention - SystemV / AMD64-ABI.
    X86SysV64, 

    X64LightCall2,
    X64LightCall3,
    X64LightCall4,

    /// Legacy calling convention, floating point arguments are passed via GP registers.
    Arm32SoftFP,
    /// Modern calling convention, uses VFP registers to pass floating point arguments.
    Arm32HardFP,

}

impl CallConv {
    pub fn is_x86_family(&self) {

    }
    pub fn is_arm_family(&self) {

    }
}

/// Function calling convention.
///
/// Function calling convention is a scheme that defines how function parameters
/// are passed and how function returns its result. AsmJit defines a variety of
/// architecture and OS specific calling conventions and also provides a compile
/// time detection to make the code-generation easier.
pub struct CallConv {
    call_conv: CallConvVariants,
    arch: ArchVariants,
    strategy: ,

    pub fn call_conv(&self) -> CallConvVariants {
        self.call_conv
    }

    pub fn set_call_conv(&self, cv: CallConvVariants) {
        self.call_conv = cv
    }

    pub fn arch(&self) -> ArchVariants {
        self.arch
    }


}