//! Count of register groups used by virtual registers.
pub const K_GROUP_VIRT: usize = 4;

/// Operand types that can be encoded in `Operand`.
enum OpType {
    // Not an operand or not initialized.
    OpNone,
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

/// Structure that allows to extract a register information based on the signature.
///
/// This information is compatible with operand's signature (32-bit integer)
/// and `RegInfo` just provides easy way to access it.
trait RegInfo {
    fn operand_type();
    fn group();
    fn r#type();
    fn size();
}

// CPP: changes 
// 1) Remove from RegInfo:
//  - signature 
//  - isValid,
//  - _getSignaturePart 
//  - setSignature
//  - reset
// Raname opType to operand_type
// Make it trait 