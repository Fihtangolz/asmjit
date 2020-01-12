pub struct Operand {

}

/// Physical/Virtual register operand
pub struct BaseReg {

}

/// Structure that allows to extract a register information based on the signature.
///
/// This information is compatible with operand's signature (32-bit integer)
/// and `RegInfo` just provides easy way to access it.
pub struct RegInfo {
    v: u32 
}

impl RegInfo {
    fn reset(&mut self) {
        self.v = 0;
    }

    fn set_signature(&mut self, signature: u32) {
        self.v = signature;
    }

    fn size() {
        
    }
}

// CPP: changes 