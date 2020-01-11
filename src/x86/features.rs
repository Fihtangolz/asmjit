struct Сpuid { 
    eax: u32, 
    ebx: u32, 
    ecx: u32, 
    edx: u32,
}

fn cpuid(eax: u32, ecx: u32) -> Cpuid {
    #[cfg(target_arch = "x86")]
    unsafe {
        "mov %%ebx, %%edi\n"
    "cpuid\n"
    "xchg %%edi, %%ebx\n" : "=a"(out->eax), "=D"(out->ebx), "=c"(out->ecx), "=d"(out->edx) : "a"(inEax), "c"(inEcx));
        asm!("mov %rbx, %rdi
            cpuid
            xchg %rdi, %rbx" 
            : "=a"(out->eax), "=D"(out->ebx), "=c"(out->ecx), "=d"(out->edx) 
            : "a"(inEax), "c"(inEcx)
            :
        );
    }

    #[cfg(target_arch = "x86_64")]
    unsafe {
        "mov %%rbx, %%rdi\n"
        "cpuid\n"
        "xchg %%rdi, %%rbx\n" : "=a"(out->eax), "=D"(out->ebx), "=c"(out->ecx), "=d"(out->edx) : "a"(inEax), "c"(inEcx));
        asm!("mov %rbx, %rdi
            cpuid
            xchg %rdi, %rbx" 
            : "=a"(out->eax), "=D"(out->ebx), "=c"(out->ecx), "=d"(out->edx) 
            : "a"(inEax), "c"(inEcx)
            :
        );
    }
}

struct Xgetbv {
    eax: u32,
    edx: u32,
}

fn xgetbv() -> {
    // Replaced, because the world is not perfect:
    //   __asm__ __volatile__("xgetbv" : "=a"(outEax), "=d"(outEdx) : "c"(inEcx));
    __asm__ __volatile__(".byte 0x0F, 0x01, 0xD0" : "=a"(outEax), "=d"(outEdx) : "c"(inEcx));
}

enum Features {
    MT,                       //!< CPU has multi-threading capabilities.
    NX,                       //!< CPU has Not-Execute-Bit aka DEP (data-execution prevention).

    3DNOW,                    //!< CPU has 3DNOW            (3DNOW base instructions) [AMD].
    3DNOW2,                   //!< CPU has 3DNOW2           (enhanced 3DNOW) [AMD].
    ADX,                      //!< CPU has ADX              (multi-precision add-carry instruction extensions).
    AESNI,                    //!< CPU has AESNI            (AES encode/decode instructions).
    ALTMOVCR8,                //!< CPU has LOCK MOV R<->CR0 (supports `MOV R<->CR8` via `LOCK MOV R<->CR0` in 32-bit mode) [AMD].
    AVX,                      //!< CPU has AVX              (advanced vector extensions).
    AVX2,                     //!< CPU has AVX2             (advanced vector extensions 2).
    AVX512_4FMAPS,            //!< CPU has AVX512_FMAPS     (FMA packed single).
    AVX512_4VNNIW,            //!< CPU has AVX512_VNNIW     (vector NN instructions word variable precision).
    AVX512_BF16,              //!< CPU has AVX512_BF16      (BFLOAT16 support instruction).
    AVX512_BITALG,            //!< CPU has AVX512_BITALG    (VPOPCNT[B|W], VPSHUFBITQMB).
    AVX512_BW,                //!< CPU has AVX512_BW        (packed BYTE|WORD).
    AVX512_CDI,               //!< CPU has AVX512_CDI       (conflict detection).
    AVX512_DQ,                //!< CPU has AVX512_DQ        (packed DWORD|QWORD).
    AVX512_ERI,               //!< CPU has AVX512_ERI       (exponential and reciprocal).
    AVX512_F,                 //!< CPU has AVX512_F         (AVX512 foundation).
    AVX512_IFMA,              //!< CPU has AVX512_IFMA      (integer fused-multiply-add using 52-bit precision).
    AVX512_PFI,               //!< CPU has AVX512_PFI       (prefetch instructions).
    AVX512_VBMI,              //!< CPU has AVX512_VBMI      (vector byte manipulation).
    AVX512_VBMI2,             //!< CPU has AVX512_VBMI2     (vector byte manipulation 2).
    AVX512_VL,                //!< CPU has AVX512_VL        (vector length extensions).
    AVX512_VNNI,              //!< CPU has AVX512_VNNI      (vector neural network instructions).
    AVX512_VP2INTERSECT,      //!< CPU has AVX512_VP2INTERSECT
    AVX512_VPOPCNTDQ,         //!< CPU has AVX512_VPOPCNTDQ (VPOPCNT[D|Q] instructions).
    BMI,                      //!< CPU has BMI              (bit manipulation instructions #1).
    BMI2,                     //!< CPU has BMI2             (bit manipulation instructions #2).
    CLDEMOTE,                 //!< CPU has CLDEMOTE         (cache line demote).
    CLFLUSH,                  //!< CPU has CLFUSH           (Cache Line flush).
    CLFLUSHOPT,               //!< CPU has CLFUSHOPT        (Cache Line flush - optimized).
    CLWB,                     //!< CPU has CLWB.
    CLZERO,                   //!< CPU has CLZERO.
    CMOV,                     //!< CPU has CMOV             (CMOV and FCMOV instructions).
    CMPXCHG16B,               //!< CPU has CMPXCHG16B       (compare-exchange 16 bytes) [X86_64].
    CMPXCHG8B,                //!< CPU has CMPXCHG8B        (compare-exchange 8 bytes).
    ENCLV,                    //!< CPU has ENCLV.
    ENQCMD,                   //!< CPU has ENQCMD           (enqueue stores).
    ERMS,                     //!< CPU has ERMS             (enhanced REP MOVSB/STOSB).
    F16C,                     //!< CPU has F16C.
    FMA,                      //!< CPU has FMA              (fused-multiply-add 3 operand form).
    FMA4,                     //!< CPU has FMA4             (fused-multiply-add 4 operand form).
    FPU,                      //!< CPU has FPU              (FPU support).
    FSGSBASE,                 //!< CPU has FSGSBASE.
    FXSR,                     //!< CPU has FXSR             (FXSAVE/FXRSTOR instructions).
    FXSROPT,                  //!< CPU has FXSROTP          (FXSAVE/FXRSTOR is optimized).
    GEODE,                    //!< CPU has GEODE extensions (3DNOW additions).
    GFNI,                     //!< CPU has GFNI             (Galois field instructions).
    HLE,                      //!< CPU has HLE.
    I486,                     //!< CPU has I486 features    (I486+ support).
    LAHFSAHF,                 //!< CPU has LAHF/SAHF        (LAHF/SAHF in 64-bit mode) [X86_64].
    LWP,                      //!< CPU has LWP              (lightweight profiling) [AMD].
    LZCNT,                    //!< CPU has LZCNT            (LZCNT instruction).
    MMX,                      //!< CPU has MMX              (MMX base instructions).
    MMX2,                     //!< CPU has MMX2             (MMX extensions or MMX2).
    kMONITOR,                 //!< CPU has MONITOR          (MONITOR/MWAIT instructions).
    MONITORX,                 //!< CPU has MONITORX         (MONITORX/MWAITX instructions).
    MOVBE,                    //!< CPU has MOVBE            (move with byte-order swap).
    MOVDIR64B,                //!< CPU has MOVDIR64B        (move 64 bytes as direct store).
    MOVDIRI,                  //!< CPU has MOVDIRI          (move dword/qword as direct store).
    MPX,                      //!< CPU has MPX              (memory protection extensions).
    MSR,                      //!< CPU has MSR              (RDMSR/WRMSR instructions).
    MSSE,                     //!< CPU has MSSE             (misaligned SSE support).
    OSXSAVE,                  //!< CPU has OSXSAVE          (XSAVE enabled by OS).
    PCLMULQDQ,                //!< CPU has PCLMULQDQ        (packed carry-less multiplication).
    PCOMMIT,                  //!< CPU has PCOMMIT          (PCOMMIT instruction).
    PCONFIG,                  //!< CPU has PCONFIG          (PCONFIG instruction).
    POPCNT,                   //!< CPU has POPCNT           (POPCNT instruction).
    PREFETCHW,                //!< CPU has PREFETCHW.
    PREFETCHWT1,              //!< CPU has PREFETCHWT1.
    RDPID,                    //!< CPU has RDPID.
    RDRAND,                   //!< CPU has RDRAND.
    RDSEED,                   //!< CPU has RDSEED.
    RDTSC,                    //!< CPU has RDTSC.
    RDTSCP,                   //!< CPU has RDTSCP.
    RTM,                      //!< CPU has RTM.
    SHA,                      //!< CPU has SHA              (SHA-1 and SHA-256 instructions).
    SKINIT,                   //!< CPU has SKINIT           (SKINIT/STGI instructions) [AMD].
    SMAP,                     //!< CPU has SMAP             (supervisor-mode access prevention).
    SMEP,                     //!< CPU has SMEP             (supervisor-mode execution prevention).
    SMX,                      //!< CPU has SMX              (safer mode extensions).
    SSE,                      //!< CPU has SSE.
    SSE2,                     //!< CPU has SSE2.
    SSE3,                     //!< CPU has SSE3.
    SSE4_1,                   //!< CPU has SSE4.1.
    SSE4_2,                   //!< CPU has SSE4.2.
    SSE4A,                    //!< CPU has SSE4A [AMD].
    SSSE3,                    //!< CPU has SSSE3.
    SVM,                      //!< CPU has SVM              (virtualization) [AMD].
    TBM,                      //!< CPU has TBM              (trailing bit manipulation) [AMD].
    TSX,                      //!< CPU has TSX.
    VAES,                     //!< CPU has VAES             (vector AES 256|512 bit support).
    VMX,                      //!< CPU has VMX              (virtualization) [INTEL].
    VPCLMULQDQ,               //!< CPU has VPCLMULQDQ       (vector PCLMULQDQ 256|512-bit support).
    WAITPKG,                  //!< CPU has WAITPKG          (UMONITOR, UMWAIT, TPAUSE).
    WBNOINVD,                 //!< CPU has WBNOINVD.
    XOP,                      //!< CPU has XOP              (XOP instructions) [AMD].
    XSAVE,                    //!< CPU has XSAVE.
    XSAVEC,                   //!< CPU has XSAVEC.
    XSAVEOPT,                 //!< CPU has XSAVEOPT.
    XSAVES,                   //!< CPU has XSAVES.
}

