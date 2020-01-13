macro_rules! define_inst {
    ($name:ident, $id:ident) => {
        fn $name(&mut self) -> Error {
            return self._emitter().emit();
        }
    };
}

trait EmitterExplicitT {
    /*
    type Error;
    // Native Registers
    //
    // Returns either GPD or GPQ register of the given `id` depending on the emitter's architecture.
    fn gpz(&self, id: u32) -> Gp { return Gp(_emitter()._gpRegInfo.signature(), id); }

    fn zax(&self) -> Gp { return Gp(_emitter()._gpRegInfo.signature(), Gp::kIdAx); }
    fn zcx(&self) -> Gp { return Gp(_emitter()._gpRegInfo.signature(), Gp::kIdCx); }
    fn zdx(&self) -> Gp { return Gp(_emitter()._gpRegInfo.signature(), Gp::kIdDx); }
    fn zbx(&self) -> Gp { return Gp(_emitter()._gpRegInfo.signature(), Gp::kIdBx); }
    fn zsp(&self) -> Gp { return Gp(_emitter()._gpRegInfo.signature(), Gp::kIdSp); }
    fn zbp(&self) -> Gp { return Gp(_emitter()._gpRegInfo.signature(), Gp::kIdBp); }
    fn zsi(&self) -> Gp { return Gp(_emitter()._gpRegInfo.signature(), Gp::kIdSi); }
    fn zdi(&self) -> Gp { return Gp(_emitter()._gpRegInfo.signature(), Gp::kIdDi); }

    // Native Pointers
    // 
    // Creates a target dependent pointer of which base register's id is `baseId`.
    inline Mem ptr_base(uint32_t baseId, int32_t off = 0, uint32_t size = 0) const noexcept {
        return Mem(Mem::Decomposed { _emitter()->_gpRegInfo.type(), baseId, 0, 0, off, size, 0 });
      }
    
    inline Mem ptr_zax(int32_t off = 0, uint32_t size = 0) const noexcept { return ptr_base(Gp::kIdAx, off, size); }
    inline Mem ptr_zcx(int32_t off = 0, uint32_t size = 0) const noexcept { return ptr_base(Gp::kIdCx, off, size); }
    inline Mem ptr_zdx(int32_t off = 0, uint32_t size = 0) const noexcept { return ptr_base(Gp::kIdDx, off, size); }
    inline Mem ptr_zbx(int32_t off = 0, uint32_t size = 0) const noexcept { return ptr_base(Gp::kIdBx, off, size); }
    inline Mem ptr_zsp(int32_t off = 0, uint32_t size = 0) const noexcept { return ptr_base(Gp::kIdSp, off, size); }
    inline Mem ptr_zbp(int32_t off = 0, uint32_t size = 0) const noexcept { return ptr_base(Gp::kIdBp, off, size); }
    inline Mem ptr_zsi(int32_t off = 0, uint32_t size = 0) const noexcept { return ptr_base(Gp::kIdSi, off, size); }
    inline Mem ptr_zdi(int32_t off = 0, uint32_t size = 0) const noexcept { return ptr_base(Gp::kIdDi, off, size); }

    // Creates an `intptr_t` memory operand depending on the current architecture.
    
    // Embed
    // Adds 8-bit integer data to the CodeBuffer.
    pub fn db(uint8_t x) -> Error { return static_cast<This*>(this)->embed(&x, 1); }
    //! Adds 16-bit integer data to the CodeBuffer.
    pub fn dw(uint16_t x) -> Error { return static_cast<This*>(this)->embed(&x, 2); }
    //! Adds 32-bit integer data to the CodeBuffer.
    pub fn dd(uint32_t x) -> Error { return static_cast<This*>(this)->embed(&x, 4); }
    //! Adds 64-bit integer data to the CodeBuffer.
    pub fn dq(uint64_t x) -> Error { return static_cast<This*>(this)->embed(&x, 8); }

    //! Adds 8-bit integer data to the CodeBuffer.
    pub fn dint8(int8_t x) -> Error { return static_cast<This*>(this)->embed(&x, sizeof(int8_t)); }
    //! Adds 8-bit integer data to the CodeBuffer.
    pub fn duint8(uint8_t x) -> Error { return static_cast<This*>(this)->embed(&x, sizeof(uint8_t)); }

    //! Adds 16-bit integer data to the CodeBuffer.
    pub fn dint16(int16_t x) -> Error { return static_cast<This*>(this)->embed(&x, sizeof(int16_t)); }
    //! Adds 16-bit integer data to the CodeBuffer.
    pub fn duint16(uint16_t x) -> Error { return static_cast<This*>(this)->embed(&x, sizeof(uint16_t)); }

    //! Adds 32-bit integer data to the CodeBuffer.
    pub fn dint32(int32_t x) -> Error { return static_cast<This*>(this)->embed(&x, sizeof(int32_t)); }
    //! Adds 32-bit integer data to the CodeBuffer.
    pub fn duint32(uint32_t x) -> Error { return static_cast<This*>(this)->embed(&x, sizeof(uint32_t)); }

    //! Adds 64-bit integer data to the CodeBuffer.
    pub fn dint64(int64_t x) -> Error { return static_cast<This*>(this)->embed(&x, sizeof(int64_t)); }
    //! Adds 64-bit integer data to the CodeBuffer.
    pub fn duint64(uint64_t x) -> Error { return static_cast<This*>(this)->embed(&x, sizeof(uint64_t)); }

    //! Adds float data to the CodeBuffer.
    pub fn dfloat(float x) -> Error { return static_cast<This*>(this)->embed(&x, sizeof(float)); }
    //! Adds double data to the CodeBuffer.
    pub fn ddouble(double x) -> Error { return static_cast<This*>(this)->embed(&x, sizeof(double)); }

    //! Adds MMX data to the CodeBuffer.
    pub fn dmm(const Data64& x) -> Error { return static_cast<This*>(this)->embed(&x, sizeof(Data64)); }
    //! Adds XMM data to the CodeBuffer.
    pub fn dxmm(const Data128& x) -> Error { return static_cast<This*>(this)->embed(&x, sizeof(Data128)); }
    //! Adds YMM data to the CodeBuffer.
    pub fn dymm(const Data256& x) -> Error { return static_cast<This*>(this)->embed(&x, sizeof(Data256)); }

    //! Adds data in a given structure instance to the CodeBuffer.
    template<typename T>
    inline Error dstruct(const T& x) { return static_cast<This*>(this)->embed(&x, uint32_t(sizeof(T))); }

    //Prefix Options

    //Base Instructions & GP Extensions
    define_inst!(cbw, Cbw);                                             // ANY       [IMPLICIT] AX      <- Sign Extend AL
    define_inst!(cdq, Cdq);                                             // ANY       [IMPLICIT] EDX:EAX <- Sign Extend EAX
    define_inst!(cdqe, Cdqe);                                           // X64       [IMPLICIT] RAX     <- Sign Extend EAX
    define_inst!(cmpxchg, Cmpxchg, Gp, Gp);                             // I486      [IMPLICIT]
    define_inst!(cmpxchg, Cmpxchg, Mem, Gp);                            // I486      [IMPLICIT]
    define_inst!(cmpxchg16b, Cmpxchg16b, Mem);                          // CMPXCHG8B [IMPLICIT] m == RDX:RAX ? m <- RCX:RBX
    define_inst!(cmpxchg8b, Cmpxchg8b, Mem);                            // CMPXCHG16B[IMPLICIT] m == EDX:EAX ? m <- ECX:EBX
    define_inst!(cpuid, Cpuid);                                         // I486      [IMPLICIT] EAX:EBX:ECX:EDX  <- CPUID[EAX:ECX]
    define_inst!(cqo, Cqo);                                             // X64       [IMPLICIT] RDX:RAX <- Sign Extend RAX
    define_inst!(cwd, Cwd);                                             // ANY       [IMPLICIT] DX:AX   <- Sign Extend AX
    define_inst!(cwde, Cwde);                                           // ANY       [IMPLICIT] EAX     <- Sign Extend AX
    define_inst!(daa, Daa);
    define_inst!(das, Das);
    define_inst!(div, Div, Gp);                                         // ANY       [IMPLICIT] {AH[Rem]: AL[Quot] <- AX / r8} {xDX[Rem]:xAX[Quot] <- DX:AX / r16|r32|r64}
    define_inst!(div, Div, Mem);                                        // ANY       [IMPLICIT] {AH[Rem]: AL[Quot] <- AX / m8} {xDX[Rem]:xAX[Quot] <- DX:AX / m16|m32|m64}
    define_inst!(idiv, Idiv, Gp);                                       // ANY       [IMPLICIT] {AH[Rem]: AL[Quot] <- AX / r8} {xDX[Rem]:xAX[Quot] <- DX:AX / r16|r32|r64}
    define_inst!(idiv, Idiv, Mem);                                      // ANY       [IMPLICIT] {AH[Rem]: AL[Quot] <- AX / m8} {xDX[Rem]:xAX[Quot] <- DX:AX / m16|m32|m64}
    define_inst!(imul, Imul, Gp);                                       // ANY       [IMPLICIT] {AX <- AL * r8} {xAX:xDX <- xAX * r16|r32|r64}
    define_inst!(imul, Imul, Mem);                                      // ANY       [IMPLICIT] {AX <- AL * m8} {xAX:xDX <- xAX * m16|m32|m64}
    define_inst!(iret, Iret);                                           // ANY       [IMPLICIT]
    define_inst!(iretd, Iretd);                                         // ANY       [IMPLICIT]
    define_inst!(iretq, Iretq);                                         // X64       [IMPLICIT]
    define_inst!(iretw, Iretw);                                         // ANY       [IMPLICIT]
    define_inst!(jecxz, Jecxz, Label);                                  // ANY       [IMPLICIT] Short jump if CX/ECX/RCX is zero.
    define_inst!(jecxz, Jecxz, Imm);                                    // ANY       [IMPLICIT] Short jump if CX/ECX/RCX is zero.
    define_inst!(jecxz, Jecxz, uint64_t);                               // ANY       [IMPLICIT] Short jump if CX/ECX/RCX is zero.
    define_inst!(lahf, Lahf);                                           // LAHFSAHF  [IMPLICIT] AH <- EFL
    define_inst!(loop, Loop, Label);                                    // ANY       [IMPLICIT] Decrement xCX; short jump if xCX != 0.
    define_inst!(loop, Loop, Imm);                                      // ANY       [IMPLICIT] Decrement xCX; short jump if xCX != 0.
    define_inst!(loop, Loop, uint64_t);                                 // ANY       [IMPLICIT] Decrement xCX; short jump if xCX != 0.
    define_inst!(loope, Loope, Label);                                  // ANY       [IMPLICIT] Decrement xCX; short jump if xCX != 0 && ZF == 1.
    define_inst!(loope, Loope, Imm);                                    // ANY       [IMPLICIT] Decrement xCX; short jump if xCX != 0 && ZF == 1.
    define_inst!(loope, Loope, uint64_t);                               // ANY       [IMPLICIT] Decrement xCX; short jump if xCX != 0 && ZF == 1.
    define_inst!(loopne, Loopne, Label);                                // ANY       [IMPLICIT] Decrement xCX; short jump if xCX != 0 && ZF == 0.
    define_inst!(loopne, Loopne, Imm);                                  // ANY       [IMPLICIT] Decrement xCX; short jump if xCX != 0 && ZF == 0.
    define_inst!(loopne, Loopne, uint64_t);                             // ANY       [IMPLICIT] Decrement xCX; short jump if xCX != 0 && ZF == 0.
    define_inst!(mul, Mul, Gp);                                         // ANY       [IMPLICIT] {AX <- AL * r8} {xDX:xAX <- xAX * r16|r32|r64}
    define_inst!(mul, Mul, Mem);                                        // ANY       [IMPLICIT] {AX <- AL * m8} {xDX:xAX <- xAX * m16|m32|m64}
    define_inst!(rdmsr, Rdmsr);                                         // ANY       [IMPLICIT]
    define_inst!(rdpmc, Rdpmc);                                         // ANY       [IMPLICIT]
    define_inst!(rdtsc, Rdtsc);                                         // RDTSC     [IMPLICIT] EDX:EAX <- CNT
    define_inst!(rdtscp, Rdtscp);                                       // RDTSCP    [IMPLICIT] EDX:EAX:EXC <- CNT
    define_inst!(ret, Ret);
    define_inst!(ret, Ret, Imm);
    define_inst!(sahf, Sahf);                                           // LAHFSAHF  [IMPLICIT] EFL <- AH
    define_inst!(syscall, Syscall);                                     // X64       [IMPLICIT]
    define_inst!(sysenter, Sysenter);                                   // X64       [IMPLICIT]
    define_inst!(sysexit, Sysexit);                                     // X64       [IMPLICIT]
    define_inst!(sysexit64, Sysexit64);                                 // X64       [IMPLICIT]
    define_inst!(sysret, Sysret);                                       // X64       [IMPLICIT]
    define_inst!(sysret64, Sysret64);                                   // X64       [IMPLICIT]
    define_inst!(wrmsr, Wrmsr);                                         // ANY       [IMPLICIT]
    define_inst!(xlatb, Xlatb);                                         // ANY       [IMPLICIT]

    //String Instruction Aliases


    //CL Instructions
    define_inst!(clzero, Clzero);                                       // CLZERO    [IMPLICIT]

    //BMI2 Instructions
    define_inst!(mulx, Mulx, Gp, Gp, Gp);                               // BMI2      [IMPLICIT]
    define_inst!(mulx, Mulx, Gp, Gp, Mem);                              // BMI2      [IMPLICIT]

    //FXSR & XSAVE Instructions
    define_inst!(xgetbv, Xgetbv);                                       // XSAVE     [IMPLICIT] EDX:EAX <- XCR[ECX]
    define_inst!(xrstor, Xrstor, Mem);                                  // XSAVE     [IMPLICIT]
    define_inst!(xrstor64, Xrstor64, Mem);                              // XSAVE+X64 [IMPLICIT]
    define_inst!(xrstors, Xrstors, Mem);                                // XSAVE     [IMPLICIT]
    define_inst!(xrstors64, Xrstors64, Mem);                            // XSAVE+X64 [IMPLICIT]
    define_inst!(xsave, Xsave, Mem);                                    // XSAVE     [IMPLICIT]
    define_inst!(xsave64, Xsave64, Mem);                                // XSAVE+X64 [IMPLICIT]
    define_inst!(xsavec, Xsavec, Mem);                                  // XSAVE     [IMPLICIT]
    define_inst!(xsavec64, Xsavec64, Mem);                              // XSAVE+X64 [IMPLICIT]
    define_inst!(xsaveopt, Xsaveopt, Mem);                              // XSAVE     [IMPLICIT]
    define_inst!(xsaveopt64, Xsaveopt64, Mem);                          // XSAVE+X64 [IMPLICIT]
    define_inst!(xsaves, Xsaves, Mem);                                  // XSAVE     [IMPLICIT]
    define_inst!(xsaves64, Xsaves64, Mem);                              // XSAVE+X64 [IMPLICIT]
    define_inst!(xsetbv, Xsetbv);                                       // XSAVE     [IMPLICIT] XCR[ECX] <- EDX:EAX

    //Monitor & MWait Instructions
    define_inst!(monitor, Monitor);
    define_inst!(monitorx, Monitorx);
    define_inst!(mwait, Mwait);
    define_inst!(mwaitx, Mwaitx);

    //MMX & SSE Instructions
    define_inst!(blendvpd, Blendvpd, Xmm, Xmm);                         // SSE4_1 [IMPLICIT]
    define_inst!(blendvpd, Blendvpd, Xmm, Mem);                         // SSE4_1 [IMPLICIT]
    define_inst!(blendvps, Blendvps, Xmm, Xmm);                         // SSE4_1 [IMPLICIT]
    define_inst!(blendvps, Blendvps, Xmm, Mem);                         // SSE4_1 [IMPLICIT]
    define_inst!(pblendvb, Pblendvb, Xmm, Xmm);                         // SSE4_1 [IMPLICIT]
    define_inst!(pblendvb, Pblendvb, Xmm, Mem);                         // SSE4_1 [IMPLICIT]
    define_inst!(maskmovq, Maskmovq, Mm, Mm);                           // SSE    [IMPLICIT]
    define_inst!(maskmovdqu, Maskmovdqu, Xmm, Xmm);                     // SSE2   [IMPLICIT]
    define_inst!(pcmpestri, Pcmpestri, Xmm, Xmm, Imm);                  // SSE4_1 [IMPLICIT]
    define_inst!(pcmpestri, Pcmpestri, Xmm, Mem, Imm);                  // SSE4_1 [IMPLICIT]
    define_inst!(pcmpestrm, Pcmpestrm, Xmm, Xmm, Imm);                  // SSE4_1 [IMPLICIT]
    define_inst!(pcmpestrm, Pcmpestrm, Xmm, Mem, Imm);                  // SSE4_1 [IMPLICIT]
    define_inst!(pcmpistri, Pcmpistri, Xmm, Xmm, Imm);                  // SSE4_1 [IMPLICIT]
    define_inst!(pcmpistri, Pcmpistri, Xmm, Mem, Imm);                  // SSE4_1 [IMPLICIT]
    define_inst!(pcmpistrm, Pcmpistrm, Xmm, Xmm, Imm);                  // SSE4_1 [IMPLICIT]
    define_inst!(pcmpistrm, Pcmpistrm, Xmm, Mem, Imm);                  // SSE4_1 [IMPLICIT]

    //SHA Instructions
    define_inst!(sha256rnds2, Sha256rnds2, Xmm, Xmm);                   // SHA [IMPLICIT]
    define_inst!(sha256rnds2, Sha256rnds2, Xmm, Mem);                   // SHA [IMPLICIT]

    //AVX, FMA, and AVX512 Instructions
    define_inst!(vmaskmovdqu, Vmaskmovdqu, Xmm, Xmm);                   // AVX  [IMPLICIT]
    define_inst!(vpcmpestri, Vpcmpestri, Xmm, Xmm, Imm);                // AVX  [IMPLICIT]
    define_inst!(vpcmpestri, Vpcmpestri, Xmm, Mem, Imm);                // AVX  [IMPLICIT]
    define_inst!(vpcmpestrm, Vpcmpestrm, Xmm, Xmm, Imm);                // AVX  [IMPLICIT]
    define_inst!(vpcmpestrm, Vpcmpestrm, Xmm, Mem, Imm);                // AVX  [IMPLICIT]
    define_inst!(vpcmpistri, Vpcmpistri, Xmm, Xmm, Imm);                // AVX  [IMPLICIT]
    define_inst!(vpcmpistri, Vpcmpistri, Xmm, Mem, Imm);                // AVX  [IMPLICIT]
    define_inst!(vpcmpistrm, Vpcmpistrm, Xmm, Xmm, Imm);                // AVX  [IMPLICIT]
    define_inst!(vpcmpistrm, Vpcmpistrm, Xmm, Mem, Imm);                // AVX  [IMPLICIT]
    */
}