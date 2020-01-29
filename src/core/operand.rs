//! Count of register groups used by virtual registers.
pub const K_GROUP_VIRT: usize = 4;

/// Operand types that can be encoded in `Operand`.
enum OpType {
    // Operand is a register.
    Reg,
    // Operand is a memory.
    Mem,
    // Operand is an immediate value.
    Imm,
    // Operand is a label.
    Label,
}

/// Constructor-less `Operand`.
///
/// Contains no initialization code and can be used safely to define an array
/// of operands that won't be initialized. This is an `Operand` compatible
/// data structure designed to be statically initialized, static const, or to
/// be used by the user to define an array of operands without having them
/// default initialized.
///
/// The key difference between `Operand` and `Operand_`:
///
/// ```
/// Operand_ xArray[10]; // Not initialized, contains garbage.
/// Operand  yArray[10]; // All operands initialized to none.
/// ```
struct Operand_ {

}

pub struct Operand {

}

/// Physical/Virtual register operand
pub struct BaseReg {

}

// Architecture neutral register types.
//
// These must be reused by any platform that contains that types. All GP
// and VEC registers are also allowed by design to be part of a BASE|INDEX
// of a memory operand.
enum RegType {
    // 8-bit low general purpose register (X86).
    Gp8Lo = 2,
    // 8-bit high general purpose register (X86).
    Gp8Hi = 3,
    // 16-bit general purpose register (X86).
    Gp16 = 4,
    // 32-bit general purpose register (X86|ARM).
    Gp32 = 5,
    // 64-bit general purpose register (X86|ARM).
    Gp64 = 6,
    // 32-bit view of a vector register (ARM).
    Vec32 = 7,
    // 64-bit view of a vector register (ARM).
    Vec64 = 8,
    // 128-bit view of a vector register (X86|ARM).
    Vec128 = 9,
    // 256-bit view of a vector register (X86).
    Vec256 = 10,
    // 512-bit view of a vector register (X86).
    Vec512 = 11,
    // 1024-bit view of a vector register (future).
    Vec1024 = 12,
    // Other0 register, should match `kOther0` group.
    Other0 = 13,
    // Other1 register, should match `kOther1` group.
    Other1 = 14,
    // Universal id of IP/PC register (if separate).
    IP = 15,
    // Start of platform dependent register types (must be honored).
    Custom = 16,
    // Maximum possible register id of all architectures.
    Max = 31
}

// Register group (architecture neutral), and some limits.
enum RegGroup {
    // General purpose register group compatible with all backends.
    Gp = 0,
    // Vector register group compatible with all backends.
    RegVec = 1,
    // Group that is architecture dependent.
    Other0 = 2,
    // Group that is architecture dependent.
    Other1 = 3,
    // Count of register groups used by virtual registers.
    Virt = 4,
    // Count of register groups used by physical registers.
    Count = 16
}

/// Structure that allows to extract a register information based on the signature.
///
/// This information is compatible with operand's signature (32-bit integer)
/// and `RegInfo` just provides easy way to access it.
pub struct RegInfo {
    op_type: OpType,
    reg_type: RegType,
    reg_group: RegGroup,
    op_size: u8,
}

// CPP: changes 
// 1) Remove from RegInfo:
//  - signature, replace by directly field
//  - isValid,
//  - _getSignaturePart 
//  - setSignature
//  - reset
// 2) Remove from RegType None and label variants 
// 3) Remove from OpType None
