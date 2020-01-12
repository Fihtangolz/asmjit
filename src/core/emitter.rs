/// Provides a base foundation to emit code - specialized by `Assembler` and
/// `BaseBuilder`
pub struct BaseEmitter {
    // See `EmitterType`.
    type: EmitterType,
    flag: EmitterFlag,
    options: ,
    code_holder: ,
    error_handler: ,
    code_info: ,
    gp_reg_info: ,
}

impl BaseEmitter {
    fn emit(uint32_t instId, o0: &Operand, o1: &Operand, o2: &Operand, o3: &Operand) -> Error {
        uint32_t opCount = 4;

        if (o3.isNone()) {
            opCount = 3;
            if (o2.isNone()) {
            opCount = 2;
            if (o1.isNone()) {
                opCount = 1;
                if (o0.isNone())
                opCount = 0;
            }
            }
        }
        
        
    }

    fn emit_prolog() -> Error {
        unimplemented!();
    }

    fn emit_epilog() -> Error {
        unimplemented!();
    }

    fn finalize() -> Error {
        unimplemented!();
    }
}

enum EmitterType {
    TypeAssembler,
    TypeBuilder,
    TypeCompiler,
}

enum EmitterFlag {
    Finalized,
    Destroyed
}