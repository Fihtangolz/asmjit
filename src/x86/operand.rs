/// Register type.
pub enum RegType {
    GpbLo, // Low GPB register (AL, BL, CL, DL, ...).
    GpbHi, // High GPB register (AH, BH, CH, DH only).
    Gpw,   // GPW register.
    Gpd,   // GPD register.
    Gpq,   // GPQ register (64-bit).
    Xmm,   // XMM register (SSE+).
    Ymm,   // YMM register (AVX+).
    Zmm,   // ZMM register (AVX512+).
    Mm,    // MMX register.
    KReg,  // K register (AVX512+).
    SReg,  // Segment register (None, ES, CS, SS, DS, FS, GS).
    CReg,  // Control register (CR).
    DReg,  // Debug register (DR).
    St,    // FPU (x87) register.
    Bnd,   // Bound register (BND).
    Rip,   // Instruction pointer (EIP, RIP).
}

/// Register group.
pub enum RegGroup {
    Gp,          // GP register group or none (universal).
    Vec,         // XMM|YMM|ZMM register group (universal).
    Mm,          // MMX register group (legacy).
    KReg,        // K register group.

    // These are not managed by BaseCompiler nor used by Func-API:
    SReg,        // Segment register group.
    CReg,        // Control register group.
    DReg,        // Debug register group.
    St,          // FPU register group.
    Bnd,         // Bound register group.
    Rip,         // Instrucion pointer (IP).
}

// Physical id (X86).
//
// \note Register indexes have been reduced to only support general purpose
// registers. There is no need to have enumerations with number suffix that
// expands to the exactly same value as the suffix value itself.
pub enum Id {
    Ax  = 0,  // Physical id of AL|AH|AX|EAX|RAX registers.
    Cx  = 1,  // Physical id of CL|CH|CX|ECX|RCX registers.
    Dx  = 2,  // Physical id of DL|DH|DX|EDX|RDX registers.
    Bx  = 3,  // Physical id of BL|BH|BX|EBX|RBX registers.
    Sp  = 4,  // Physical id of SPL|SP|ESP|RSP registers.
    Bp  = 5,  // Physical id of BPL|BP|EBP|RBP registers.
    Si  = 6,  // Physical id of SIL|SI|ESI|RSI registers.
    Di  = 7,  // Physical id of DIL|DI|EDI|RDI registers.
    R8  = 8,  // Physical id of R8B|R8W|R8D|R8 registers (64-bit only).
    R9  = 9,  // Physical id of R9B|R9W|R9D|R9 registers (64-bit only).
    R10 = 10, // Physical id of R10B|R10W|R10D|R10 registers (64-bit only).
    R11 = 11, // Physical id of R11B|R11W|R11D|R11 registers (64-bit only).
    R12 = 12, // Physical id of R12B|R12W|R12D|R12 registers (64-bit only).
    R13 = 13, // Physical id of R13B|R13W|R13D|R13 registers (64-bit only).
    R14 = 14, // Physical id of R14B|R14W|R14D|R14 registers (64-bit only).
    R15 = 15  // Physical id of R15B|R15W|R15D|R15 registers (64-bit only).
}

pub struct Gp {

}

// CPP: changes 
// 1) Remove None from RegType
//  
