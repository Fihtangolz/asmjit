pub struct BaseEmitter {
    type: EmitterType,
    flag: EmitterFlag,
    options: ,
    code_holder: ,
    error_handler: ,
    code_info: ,
    gp_reg_info: ,
}

impl BaseEmitter {
    
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