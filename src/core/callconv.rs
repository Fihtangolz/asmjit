use super::arch::ArchVariants;
use super::operand::K_GROUP_VIRT;

pub enum CallConvVariants {
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

impl CallConvVariants {
    pub fn is_x86_family(&self) -> bool {
        use CallConvVariants::*;
        match self {
            X86CDecl 
            | X86StdCall 
            | X86MsThisCall
            | X86MsFastCall
            | X86GccFastCall 
            | X86GccRegParm1 
            | X86GccRegParm2
            | X86GccRegParm3
            | X86LightCall2
            | X86LightCall3
            | X86LightCall4 => true,
            _ => false
        }
    }
    pub fn is_arm_family(&self) -> bool {
        use CallConvVariants::*;
        match self {
            Arm32SoftFP | Arm32HardFP => true,
            _ => false,
        }
    }
}

/// Strategy used to assign registers to function arguments.
///
/// This is AsmJit specific. It basically describes how AsmJit should convert
/// the function arguments defined by `FuncSignature` into register IDs and
/// stack offsets. The default strategy `kStrategyDefault` assigns registers
/// and then stack whereas `kStrategyWin64` strategy does register shadowing
/// as defined by WIN64 calling convention - it applies to 64-bit calling
/// conventions only.
pub enum Strategy {
    Default,            // Default register assignment strategy.
    Win64,              // WIN64 specific register assignment strategy.
}

/// Function calling convention.
///
/// Function calling convention is a scheme that defines how function parameters
/// are passed and how function returns its result. AsmJit defines a variety of
/// architecture and OS specific calling conventions and also provides a compile
/// time detection to make the code-generation easier.
pub struct CallConv {
    /// Calling convention id, see `Id`.
    call_conv: CallConvVariants,
    /// Calling convention id, see `Id`.
    arch: ArchVariants,
    /// Register assignment strategy.
    strategy: Strategy,
    /// Flags.
    flags: Flags,
    
    /// Red zone size (AMD64 == 128 bytes).
    red_zone_size: u8,
    /// Spill zone size (WIN64 == 32 bytes).
    spill_zone_size: u8,
    /// Natural stack alignment as defined by OS/ABI.
    natural_stack_alignment: u8,

    /// Mask of all passed registers, per group.
    passed_regs: [u32; K_GROUP_VIRT],
    /// Mask of all preserved registers, per group.
    preserved_regs: [u32; K_GROUP_VIRT],

    /// Passed registers' order, per group.
    passed_order: [u32; K_GROUP_VIRT],
}

/// Calling convention flags.
bitflags! {
    pub struct Flags: u32 {
        /// Callee is responsible for cleaning up the stack.
        const CALLEE_POPS_STACK = 0b00000001;   
        /// Pass F32 and F64 arguments by VEC128 register.
        const PASS_FLOATS_BY_VEC = 0b00000010;   
        /// This is a '__vectorcall' calling convention.
        const VECTOR_CALL = 0b00000100;        
        /// Pass vector arguments indirectly (as a pointer).
        const INDIRECT_VEC_ARGS = 0b00001000;   
    }
}


impl CallConv {
    // Returns the calling convention id, see `Id`.
    pub fn call_conv(&self) -> &CallConvVariants {
        &self.call_conv
    }
    // Sets the calling convention id, see `Id`.
    pub fn set_call_conv(&mut self, cv: CallConvVariants) {
        self.call_conv = cv
    }
    
    // Returns the calling function architecture id.
    pub fn arch(&self) -> &ArchVariants {
        &self.arch
    }
    // Sets the calling function architecture id.
    pub fn set_arch(&mut self, arch: ArchVariants) {
        self.arch = arch;
    }
    
    // Returns the strategy used to assign registers to arguments, see `Strategy`.
    pub fn strategy(&self) -> &Strategy { 
        &self.strategy 
    }
    // Sets the strategy used to assign registers to arguments, see `Strategy`.
    pub fn set_strategy(&mut self, strategy: Strategy) { 
        self.strategy = strategy
    }
    
    // Tests whether the calling convention has the given `flag` set.
    pub fn has_flag(&mut self, flag: Flags) -> bool { 
        self.flags.contains(flag)
    }
    // Returns the calling convention flags, see `Flags`.
    pub fn flags(&self) -> Flags { 
        self.flags 
    }
    // Adds the calling convention flags, see `Flags`.
    pub fn set_flags(&mut self) { 
        unimplemented!(); 
    }
    // Adds the calling convention flags, see `Flags`.
    pub fn add_flags(&mut self) { 
        unimplemented!();
    }

    // Tests whether this calling convention specifies 'RedZone'.
    pub fn has_red_zone(&self) -> bool { 
        self.red_zone_size != 0
    }
    // Tests whether this calling convention specifies 'SpillZone'.
    pub fn has_spill_zone(&self) -> bool { 
        self.spill_zone_size != 0 
    }

    // Returns size of 'RedZone'.
    pub fn red_zone_size(&self) -> u8 { 
        self.red_zone_size 
    }
    // Returns size of 'SpillZone'.
    pub fn spill_zone_size(&self) -> u8 { 
        self.spill_zone_size 
    }

    // Sets size of 'RedZone'.
    pub fn set_red_zone_size(&mut self, size: u8) { 
        self.red_zone_size = size
    }
    // Sets size of 'SpillZone'.
    pub fn set_spill_zone_size(&mut self, size: u8) { 
        self.spill_zone_size = size
    }

    // Returns a natural stack alignment.
    pub fn natural_stack_alignment(&self) -> u8 { 
        self.natural_stack_alignment
    }
    // Sets a natural stack alignment.
    //
    // This function can be used to override the default stack alignment in case
    // that you know that it's alignment is different. For example it allows to
    // implement custom calling conventions that guarantee higher stack alignment.
    pub fn set_natural_stack_alignment(&mut self, size: u8) { 
        self.natural_stack_alignment = size 
    }

    pub fn passed_order() {
        unimplemented!();
    }
    
    pub fn passed_regs() {
        unimplemented!();
    }

    pub fn set_passed_packed() {
        unimplemented!();
    }

    pub fn set_passed_to_none() {
        unimplemented!();
    }

    pub fn set_passed_order() {
        unimplemented!();
    }

    pub fn preserved_regs() -> u32 {
        unimplemented!();
    }
    
    pub fn set_preserved_regs() {
        unimplemented!();
    }
}