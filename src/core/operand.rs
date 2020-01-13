//! Count of register groups used by virtual registers.
pub const K_GROUP_VIRT: usize = 4;

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
    fn type();
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