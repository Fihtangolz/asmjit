use super::arch::ArchInfo;
use super::callconv::CallConvVariants;

// Basic information about a code (or target). It describes its architecture,
// code generation mode (or optimization level), and base address.
pub struct CodeInfo {
    // Architecture information.
    arch_info: ArchInfo,
    // Natural stack alignment (ARCH+OS).
    stack_alignment: u8,
    // Default CDECL calling convention.
    cdecl_call_conv: CallConvVariants,
    // Default STDCALL calling convention.
    std_call_conv: CallConvVariants,
    // Default FASTCALL calling convention.
    fast_call_conv: CallConvVariants,
    // Base address.
    base_address: u8,
}

impl CodeInfo {
    pub fn new(
        arch_info: ArchInfo, 
        stack_alignment: u8,
        cdecl_call_conv: CallConvVariants,
        std_call_conv: CallConvVariants,
        fast_call_conv: CallConvVariants,
        base_address: u8
    ) -> Self {
        Self {
            arch_info: arch_info,
            stack_alignment: stack_alignment,
            cdecl_call_conv: cdecl_call_conv,
            std_call_conv: std_call_conv,
            fast_call_conv: fast_call_conv,
            base_address: base_address,
        }
    }
    pub fn arch_info(&self) -> ArchInfo {
        self.arch_info()
    }
    pub fn stack_alignment(&self) -> u8 {
        self.stack_alignment
    }
    pub fn cdecl_call_conv(&self) -> CallConvVariants {
        self.cdecl_call_conv
    }
    pub fn std_call_conv(&self) -> CallConvVariants {
        self.std_call_conv
    }
    pub fn fast_call_conv(&self) -> CallConvVariants {
        self.fast_call_conv
    }
    pub fn base_address(&self) -> u8 {
        self.base_address
    }
}

enum TargetType {
    TargetJit
}

// Target is an abstract class that describes a machine code target.
pub struct Target {
    // Tartget type.
    target_type: TargetType,
    // Basic information about the Runtime's code.
    code_info: CodeInfo,
}

impl Target {
    pub fn new(target_type: TargetType, code_info: CodeInfo) -> Self {
        Self{
            target_type: target_type,
            code_info: code_info,
        }
    }
    pub fn target_type(&self) -> TargetType {
        self.target_type
    }
    pub fn code_info(&self) -> CodeInfo {
        self.code_info
    }
}