/// Register type.
pub enum RegType {
    None,  // No register type or invalid register.
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