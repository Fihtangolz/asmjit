pub enum ArchVariants {
    /// Unknown architecture
    UNKNOWN,
    /// X86 architectures.
    /// X86 architecture (32-bit).
    X86,                        
    /// X64 architecture (64-bit) (AMD64).
    X64,                        
    /// ARM architectures.
    /// ARM 32-bit architecture (AArch32/ARM/THUMB).
    A32,                        
    /// ARM 64-bit architecture (AArch64).
    A64,                        
}

pub enum ArchSubType {
    X86_AVX,
    X86_AVX2,
    X86_AVX512,
    86_AVX512VL,
    A32_THUMB,
}

impl ArchVariants {
    /// Tests whether this architecture is ARM32 or ARM64.
    pub fn is_arm_family(&self) -> bool {
        match self {
            ArchVariants::A32 | ArchVariants::A64 => true,
            _ => false
        }
    }
    /// Tests whether this architecture is X86, X64.
    pub fn is_x86_family(&self) -> bool {
        match self {
            ArchVariants::X86 | ArchVariants::X64 => true,
            _ => false
        }  
    }
}

type GpSize = u32;

//TODO: wrap on C's enum 
pub struct ArchInfo {
    /// Architecture id.
    arch: ArchVariants,
    /// Architecture sub-id.
    sub_arch: ArchSubType,
    /// Default size of a general purpose register.
    gp_size: GpSize
    /// Count of all general purpose registers. 
    gp_count: u32

    /// Architecture signature 
    signature: u32,
}

/// Information about all architecture registers.
pub struct ArchRegs {
    pub fn init(&self, arch: ArchVariants, sub_arch: ArchSubType) {
        
    }

    pub fn reset(&self) {
        self.signature = 0
    }

    /// Returns the architecture id, see `Id`.
    pub fn arch_id(&self) -> ArchVariants {
        self.arch
    }

    /// Returns the architecture sub-id, see `SubType`.
    ///
    /// X86 & X64
    /// ---------
    ///
    /// Architecture subtype describe the highest instruction-set level that can
    /// be used.
    ///
    /// A32 & A64
    /// ---------
    ///
    /// Architecture mode means the instruction encoding to be used when generating
    /// machine code, thus mode can be used to force generation of THUMB and THUMBv2
    /// encoding or regular ARM encoding.
    pub fn arch_sub_id(&self) -> ArchSubType {
        self.sub_arch
    }

    /// Returns the native size of a general-purpose register.
    pub fn gp_size() -> u32 {
        self.gp_size
    }

    /// Returns number of general-purpose registers.
    pub fn gp_count() -> u32 {
        self.gp_count
    }
}

impl GpSize {
    /// Tests whether this architecture is 32-bit.
    pub fn is_32bit(&self) -> bool {
        self == 4
    }
    /// Tests whether this architecture is 64-bit.
    pub fn is_64bit(&self) -> bool {
        self == 8
    }
}

/// Information about all architecture registers.
pub struct ArchRegs {
    /// Register information and signatures indexed by `BaseReg::RegType`.
    reg_info: [RegInfo; 32]
    /// Count (maximum) of  registers per `BaseReg::RegType`.
    reg_count: [u8; 32],
    /// Converts RegType to TypeId, see `Type::Id`.
}