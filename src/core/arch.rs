use std::mem;
use super::operand::RegInfo;

pub enum ArchVariants {
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
    NONE,
    X86Avx,
    X86Avx2,
    X86Avx512,
    X86Avx512vl,
    A32Thumb,
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

pub struct ArchInfo {
    /// Architecture id.
    arch: ArchVariants,
    /// Architecture sub-id.
    sub_arch: ArchSubType,
    /// Default size of a general purpose register.
    gp_size: u32,
    /// Count of all general purpose registers. 
    gp_count: u32,
}

/// Information about all architecture registers.
impl ArchInfo {
    fn from_host() -> Option<Self> {
        let mut arch: Option<ArchVariants> = None;

        #[cfg(target_arch="x86")] {
            arch = Some(ArchVariants::X86);   
        }

        #[cfg(target_arch="x86_64")] {
            arch = Some(ArchVariants::X64);   
        }
        
        #[cfg(target_arch="arm")] {
            arch = Some(ArchVariants::A32);
        }
        
        #[cfg(target_arch="aarch64")] {
            arch = Some(ArchVariants::A64);
        }

        arch.map(|x| {
            //FIX: add detect sub arch type
            Self::new(x, ArchSubType::NONE)
        })
    }

    pub fn new(arch: ArchVariants, sub_arch: ArchSubType) -> Self {
        let gp_size: u32; 
        let gp_count: u32;

        match arch {
            ArchVariants::X86 => {
                gp_size = 4;
                gp_count = 8;
            },
            ArchVariants::X64 => {
                gp_size = 8;
                gp_count = 16;
            },
            ArchVariants::A32 => {
                gp_size = 4;
                gp_count = 16;
            },
            ArchVariants::A64 => {
                gp_size = 8;
                gp_count = 32;
            },
        }

        Self {
            arch: arch,
            sub_arch: sub_arch,
            gp_size: gp_size,
            gp_count: gp_count,
        }
    }

    /// Returns the architecture id, see `Id`.
    pub fn arch_id(&self) -> &ArchVariants {
        &self.arch
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
    pub fn arch_sub_id(&self) -> &ArchSubType {
        &self.sub_arch
    }

    /// Returns the native size of a general-purpose register.
    pub fn gp_size(&self) -> u32 {
        self.gp_size
    }

    /// Returns number of general-purpose registers.
    pub fn gp_count(&self) -> u32 {
        self.gp_count
    }
}

impl ArchInfo {
    /// Tests whether this architecture is 32-bit.
    pub fn is_32bit(&self) -> bool {
        self.gp_size == 4
    }
    /// Tests whether this architecture is 64-bit.
    pub fn is_64bit(&self) -> bool {
        self.gp_size == 8
    }
}

/// Information about all architecture registers.
pub struct ArchRegs<RI: RegInfo> {
    /// Register information and signatures indexed by `BaseReg::RegType`.
    reg_info: [RI; 32],
    /// Count (maximum) of  registers per `BaseReg::RegType`.
    reg_count: [u8; 32],
    // Converts RegType to TypeId, see `Type::Id`.
    reg_type_to_type_id: [u8; 32],
}

// pub fn type_id_to_reg_info(arch: ArchVariants, type_id_in_out: u32, reg_info: RegInfo) {
//     if arch.is_x86_family() {
        
//     }
// }

