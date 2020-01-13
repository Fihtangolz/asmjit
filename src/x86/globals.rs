/// Instruction id.
enum Instruction {
    Aaa,                              // Instruction 'aaa' (X86).
    Aad,                              // Instruction 'aad' (X86).
    Aam,                              // Instruction 'aam' (X86).
    Aas,                              // Instruction 'aas' (X86).
    Adc,                              // Instruction 'adc'.
    Adcx,                             // Instruction 'adcx' {ADX}.
    Add,                              // Instruction 'add'.
    Addpd,                            // Instruction 'addpd' {SSE2}.
    Addps,                            // Instruction 'addps' {SSE}.
    Addsd,                            // Instruction 'addsd' {SSE2}.
    Addss,                            // Instruction 'addss' {SSE}.
    Addsubpd,                         // Instruction 'addsubpd' {SSE3}.
    Addsubps,                         // Instruction 'addsubps' {SSE3}.
    Adox,                             // Instruction 'adox' {ADX}.
    Aesdec,                           // Instruction 'aesdec' {AESNI}.
    Aesdeclast,                       // Instruction 'aesdeclast' {AESNI}.
    Aesenc,                           // Instruction 'aesenc' {AESNI}.
    Aesenclast,                       // Instruction 'aesenclast' {AESNI}.
    Aesimc,                           // Instruction 'aesimc' {AESNI}.
    Aeskeygenassist,                  // Instruction 'aeskeygenassist' {AESNI}.
    And,                              // Instruction 'and'.
    Andn,                             // Instruction 'andn' {BMI}.
    Andnpd,                           // Instruction 'andnpd' {SSE2}.
    Andnps,                           // Instruction 'andnps' {SSE}.
    Andpd,                            // Instruction 'andpd' {SSE2}.
    Andps,                            // Instruction 'andps' {SSE}.
    Arpl,                             // Instruction 'arpl' (X86).
    Bextr,                            // Instruction 'bextr' {BMI}.
    Blcfill,                          // Instruction 'blcfill' {TBM}.
    Blci,                             // Instruction 'blci' {TBM}.
    Blcic,                            // Instruction 'blcic' {TBM}.
    Blcmsk,                           // Instruction 'blcmsk' {TBM}.
    Blcs,                             // Instruction 'blcs' {TBM}.
    Blendpd,                          // Instruction 'blendpd' {SSE4_1}.
    Blendps,                          // Instruction 'blendps' {SSE4_1}.
    Blendvpd,                         // Instruction 'blendvpd' {SSE4_1}.
    Blendvps,                         // Instruction 'blendvps' {SSE4_1}.
    Blsfill,                          // Instruction 'blsfill' {TBM}.
    Blsi,                             // Instruction 'blsi' {BMI}.
    Blsic,                            // Instruction 'blsic' {TBM}.
    Blsmsk,                           // Instruction 'blsmsk' {BMI}.
    Blsr,                             // Instruction 'blsr' {BMI}.
    Bndcl,                            // Instruction 'bndcl' {MPX}.
    Bndcn,                            // Instruction 'bndcn' {MPX}.
    Bndcu,                            // Instruction 'bndcu' {MPX}.
    Bndldx,                           // Instruction 'bndldx' {MPX}.
    Bndmk,                            // Instruction 'bndmk' {MPX}.
    Bndmov,                           // Instruction 'bndmov' {MPX}.
    Bndstx,                           // Instruction 'bndstx' {MPX}.
    Bound,                            // Instruction 'bound' (X86).
    Bsf,                              // Instruction 'bsf'.
    Bsr,                              // Instruction 'bsr'.
    Bswap,                            // Instruction 'bswap'.
    Bt,                               // Instruction 'bt'.
    Btc,                              // Instruction 'btc'.
    Btr,                              // Instruction 'btr'.
    Bts,                              // Instruction 'bts'.
    Bzhi,                             // Instruction 'bzhi' {BMI2}.
    Call,                             // Instruction 'call'.
    Cbw,                              // Instruction 'cbw'.
    Cdq,                              // Instruction 'cdq'.
    Cdqe,                             // Instruction 'cdqe' (X64).
    Clac,                             // Instruction 'clac' {SMAP}.
    Clc,                              // Instruction 'clc'.
    Cld,                              // Instruction 'cld'.
    Cldemote,                         // Instruction 'cldemote' {CLDEMOTE}.
    Clflush,                          // Instruction 'clflush' {CLFLUSH}.
    Clflushopt,                       // Instruction 'clflushopt' {CLFLUSHOPT}.
    Clgi,                             // Instruction 'clgi' {SVM}.
    Cli,                              // Instruction 'cli'.
    Clts,                             // Instruction 'clts'.
    Clwb,                             // Instruction 'clwb' {CLWB}.
    Clzero,                           // Instruction 'clzero' {CLZERO}.
    Cmc,                              // Instruction 'cmc'.
    Cmova,                            // Instruction 'cmova' {CMOV}.
    Cmovae,                           // Instruction 'cmovae' {CMOV}.
    Cmovb,                            // Instruction 'cmovb' {CMOV}.
    Cmovbe,                           // Instruction 'cmovbe' {CMOV}.
    Cmovc,                            // Instruction 'cmovc' {CMOV}.
    Cmove,                            // Instruction 'cmove' {CMOV}.
    Cmovg,                            // Instruction 'cmovg' {CMOV}.
    Cmovge,                           // Instruction 'cmovge' {CMOV}.
    Cmovl,                            // Instruction 'cmovl' {CMOV}.
    Cmovle,                           // Instruction 'cmovle' {CMOV}.
    Cmovna,                           // Instruction 'cmovna' {CMOV}.
    Cmovnae,                          // Instruction 'cmovnae' {CMOV}.
    Cmovnb,                           // Instruction 'cmovnb' {CMOV}.
    Cmovnbe,                          // Instruction 'cmovnbe' {CMOV}.
    Cmovnc,                           // Instruction 'cmovnc' {CMOV}.
    Cmovne,                           // Instruction 'cmovne' {CMOV}.
    Cmovng,                           // Instruction 'cmovng' {CMOV}.
    Cmovnge,                          // Instruction 'cmovnge' {CMOV}.
    Cmovnl,                           // Instruction 'cmovnl' {CMOV}.
    Cmovnle,                          // Instruction 'cmovnle' {CMOV}.
    Cmovno,                           // Instruction 'cmovno' {CMOV}.
    Cmovnp,                           // Instruction 'cmovnp' {CMOV}.
    Cmovns,                           // Instruction 'cmovns' {CMOV}.
    Cmovnz,                           // Instruction 'cmovnz' {CMOV}.
    Cmovo,                            // Instruction 'cmovo' {CMOV}.
    Cmovp,                            // Instruction 'cmovp' {CMOV}.
    Cmovpe,                           // Instruction 'cmovpe' {CMOV}.
    Cmovpo,                           // Instruction 'cmovpo' {CMOV}.
    Cmovs,                            // Instruction 'cmovs' {CMOV}.
    Cmovz,                            // Instruction 'cmovz' {CMOV}.
    Cmp,                              // Instruction 'cmp'.
    Cmppd,                            // Instruction 'cmppd' {SSE2}.
    Cmpps,                            // Instruction 'cmpps' {SSE}.
    Cmps,                             // Instruction 'cmps'.
    Cmpsd,                            // Instruction 'cmpsd' {SSE2}.
    Cmpss,                            // Instruction 'cmpss' {SSE}.
    Cmpxchg,                          // Instruction 'cmpxchg' {I486}.
    Cmpxchg16b,                       // Instruction 'cmpxchg16b' {CMPXCHG16B} (X64).
    Cmpxchg8b,                        // Instruction 'cmpxchg8b' {CMPXCHG8B}.
    Comisd,                           // Instruction 'comisd' {SSE2}.
    Comiss,                           // Instruction 'comiss' {SSE}.
    Cpuid,                            // Instruction 'cpuid' {I486}.
    Cqo,                              // Instruction 'cqo' (X64).
    Crc32,                            // Instruction 'crc32' {SSE4_2}.
    Cvtdq2pd,                         // Instruction 'cvtdq2pd' {SSE2}.
    Cvtdq2ps,                         // Instruction 'cvtdq2ps' {SSE2}.
    Cvtpd2dq,                         // Instruction 'cvtpd2dq' {SSE2}.
    Cvtpd2pi,                         // Instruction 'cvtpd2pi' {SSE2}.
    Cvtpd2ps,                         // Instruction 'cvtpd2ps' {SSE2}.
    Cvtpi2pd,                         // Instruction 'cvtpi2pd' {SSE2}.
    Cvtpi2ps,                         // Instruction 'cvtpi2ps' {SSE}.
    Cvtps2dq,                         // Instruction 'cvtps2dq' {SSE2}.
    Cvtps2pd,                         // Instruction 'cvtps2pd' {SSE2}.
    Cvtps2pi,                         // Instruction 'cvtps2pi' {SSE}.
    Cvtsd2si,                         // Instruction 'cvtsd2si' {SSE2}.
    Cvtsd2ss,                         // Instruction 'cvtsd2ss' {SSE2}.
    Cvtsi2sd,                         // Instruction 'cvtsi2sd' {SSE2}.
    Cvtsi2ss,                         // Instruction 'cvtsi2ss' {SSE}.
    Cvtss2sd,                         // Instruction 'cvtss2sd' {SSE2}.
    Cvtss2si,                         // Instruction 'cvtss2si' {SSE}.
    Cvttpd2dq,                        // Instruction 'cvttpd2dq' {SSE2}.
    Cvttpd2pi,                        // Instruction 'cvttpd2pi' {SSE2}.
    Cvttps2dq,                        // Instruction 'cvttps2dq' {SSE2}.
    Cvttps2pi,                        // Instruction 'cvttps2pi' {SSE}.
    Cvttsd2si,                        // Instruction 'cvttsd2si' {SSE2}.
    Cvttss2si,                        // Instruction 'cvttss2si' {SSE}.
    Cwd,                              // Instruction 'cwd'.
    Cwde,                             // Instruction 'cwde'.
    Daa,                              // Instruction 'daa' (X86).
    Das,                              // Instruction 'das' (X86).
    Dec,                              // Instruction 'dec'.
    Div,                              // Instruction 'div'.
    Divpd,                            // Instruction 'divpd' {SSE2}.
    Divps,                            // Instruction 'divps' {SSE}.
    Divsd,                            // Instruction 'divsd' {SSE2}.
    Divss,                            // Instruction 'divss' {SSE}.
    Dppd,                             // Instruction 'dppd' {SSE4_1}.
    Dpps,                             // Instruction 'dpps' {SSE4_1}.
    Emms,                             // Instruction 'emms' {MMX}.
    Enqcmd,                           // Instruction 'enqcmd' {ENQCMD}.
    Enqcmds,                          // Instruction 'enqcmds' {ENQCMD}.
    Enter,                            // Instruction 'enter'.
    Extractps,                        // Instruction 'extractps' {SSE4_1}.
    Extrq,                            // Instruction 'extrq' {SSE4A}.
    F2xm1,                            // Instruction 'f2xm1'.
    Fabs,                             // Instruction 'fabs'.
    Fadd,                             // Instruction 'fadd'.
    Faddp,                            // Instruction 'faddp'.
    Fbld,                             // Instruction 'fbld'.
    Fbstp,                            // Instruction 'fbstp'.
    Fchs,                             // Instruction 'fchs'.
    Fclex,                            // Instruction 'fclex'.
    Fcmovb,                           // Instruction 'fcmovb' {CMOV}.
    Fcmovbe,                          // Instruction 'fcmovbe' {CMOV}.
    Fcmove,                           // Instruction 'fcmove' {CMOV}.
    Fcmovnb,                          // Instruction 'fcmovnb' {CMOV}.
    Fcmovnbe,                         // Instruction 'fcmovnbe' {CMOV}.
    Fcmovne,                          // Instruction 'fcmovne' {CMOV}.
    Fcmovnu,                          // Instruction 'fcmovnu' {CMOV}.
    Fcmovu,                           // Instruction 'fcmovu' {CMOV}.
    Fcom,                             // Instruction 'fcom'.
    Fcomi,                            // Instruction 'fcomi'.
    Fcomip,                           // Instruction 'fcomip'.
    Fcomp,                            // Instruction 'fcomp'.
    Fcompp,                           // Instruction 'fcompp'.
    Fcos,                             // Instruction 'fcos'.
    Fdecstp,                          // Instruction 'fdecstp'.
    Fdiv,                             // Instruction 'fdiv'.
    Fdivp,                            // Instruction 'fdivp'.
    Fdivr,                            // Instruction 'fdivr'.
    Fdivrp,                           // Instruction 'fdivrp'.
    Femms,                            // Instruction 'femms' {3DNOW}.
    Ffree,                            // Instruction 'ffree'.
    Fiadd,                            // Instruction 'fiadd'.
    Ficom,                            // Instruction 'ficom'.
    Ficomp,                           // Instruction 'ficomp'.
    Fidiv,                            // Instruction 'fidiv'.
    Fidivr,                           // Instruction 'fidivr'.
    Fild,                             // Instruction 'fild'.
    Fimul,                            // Instruction 'fimul'.
    Fincstp,                          // Instruction 'fincstp'.
    Finit,                            // Instruction 'finit'.
    Fist,                             // Instruction 'fist'.
    Fistp,                            // Instruction 'fistp'.
    Fisttp,                           // Instruction 'fisttp' {SSE3}.
    Fisub,                            // Instruction 'fisub'.
    Fisubr,                           // Instruction 'fisubr'.
    Fld,                              // Instruction 'fld'.
    Fld1,                             // Instruction 'fld1'.
    Fldcw,                            // Instruction 'fldcw'.
    Fldenv,                           // Instruction 'fldenv'.
    Fldl2e,                           // Instruction 'fldl2e'.
    Fldl2t,                           // Instruction 'fldl2t'.
    Fldlg2,                           // Instruction 'fldlg2'.
    Fldln2,                           // Instruction 'fldln2'.
    Fldpi,                            // Instruction 'fldpi'.
    Fldz,                             // Instruction 'fldz'.
    Fmul,                             // Instruction 'fmul'.
    Fmulp,                            // Instruction 'fmulp'.
    Fnclex,                           // Instruction 'fnclex'.
    Fninit,                           // Instruction 'fninit'.
    Fnop,                             // Instruction 'fnop'.
    Fnsave,                           // Instruction 'fnsave'.
    Fnstcw,                           // Instruction 'fnstcw'.
    Fnstenv,                          // Instruction 'fnstenv'.
    Fnstsw,                           // Instruction 'fnstsw'.
    Fpatan,                           // Instruction 'fpatan'.
    Fprem,                            // Instruction 'fprem'.
    Fprem1,                           // Instruction 'fprem1'.
    Fptan,                            // Instruction 'fptan'.
    Frndint,                          // Instruction 'frndint'.
    Frstor,                           // Instruction 'frstor'.
    Fsave,                            // Instruction 'fsave'.
    Fscale,                           // Instruction 'fscale'.
    Fsin,                             // Instruction 'fsin'.
    Fsincos,                          // Instruction 'fsincos'.
    Fsqrt,                            // Instruction 'fsqrt'.
    Fst,                              // Instruction 'fst'.
    Fstcw,                            // Instruction 'fstcw'.
    Fstenv,                           // Instruction 'fstenv'.
    Fstp,                             // Instruction 'fstp'.
    Fstsw,                            // Instruction 'fstsw'.
    Fsub,                             // Instruction 'fsub'.
    Fsubp,                            // Instruction 'fsubp'.
    Fsubr,                            // Instruction 'fsubr'.
    Fsubrp,                           // Instruction 'fsubrp'.
    Ftst,                             // Instruction 'ftst'.
    Fucom,                            // Instruction 'fucom'.
    Fucomi,                           // Instruction 'fucomi'.
    Fucomip,                          // Instruction 'fucomip'.
    Fucomp,                           // Instruction 'fucomp'.
    Fucompp,                          // Instruction 'fucompp'.
    Fwait,                            // Instruction 'fwait'.
    Fxam,                             // Instruction 'fxam'.
    Fxch,                             // Instruction 'fxch'.
    Fxrstor,                          // Instruction 'fxrstor' {FXSR}.
    Fxrstor64,                        // Instruction 'fxrstor64' {FXSR} (X64).
    Fxsave,                           // Instruction 'fxsave' {FXSR}.
    Fxsave64,                         // Instruction 'fxsave64' {FXSR} (X64).
    Fxtract,                          // Instruction 'fxtract'.
    Fyl2x,                            // Instruction 'fyl2x'.
    Fyl2xp1,                          // Instruction 'fyl2xp1'.
    Getsec,                           // Instruction 'getsec' {SMX}.
    Gf2p8affineinvqb,                 // Instruction 'gf2p8affineinvqb' {GFNI}.
    Gf2p8affineqb,                    // Instruction 'gf2p8affineqb' {GFNI}.
    Gf2p8mulb,                        // Instruction 'gf2p8mulb' {GFNI}.
    Haddpd,                           // Instruction 'haddpd' {SSE3}.
    Haddps,                           // Instruction 'haddps' {SSE3}.
    Hlt,                              // Instruction 'hlt'.
    Hsubpd,                           // Instruction 'hsubpd' {SSE3}.
    Hsubps,                           // Instruction 'hsubps' {SSE3}.
    Idiv,                             // Instruction 'idiv'.
    Imul,                             // Instruction 'imul'.
    In,                               // Instruction 'in'.
    Inc,                              // Instruction 'inc'.
    Ins,                              // Instruction 'ins'.
    Insertps,                         // Instruction 'insertps' {SSE4_1}.
    Insertq,                          // Instruction 'insertq' {SSE4A}.
    Int,                              // Instruction 'int'.
    Int3,                             // Instruction 'int3'.
    Into,                             // Instruction 'into' (X86).
    Invd,                             // Instruction 'invd' {I486}.
    Invept,                           // Instruction 'invept' {VMX}.
    Invlpg,                           // Instruction 'invlpg' {I486}.
    Invlpga,                          // Instruction 'invlpga' {SVM}.
    Invpcid,                          // Instruction 'invpcid' {I486}.
    Invvpid,                          // Instruction 'invvpid' {VMX}.
    Iret,                             // Instruction 'iret'.
    Iretd,                            // Instruction 'iretd'.
    Iretq,                            // Instruction 'iretq' (X64).
    Iretw,                            // Instruction 'iretw'.
    Ja,                               // Instruction 'ja'.
    Jae,                              // Instruction 'jae'.
    Jb,                               // Instruction 'jb'.
    Jbe,                              // Instruction 'jbe'.
    Jc,                               // Instruction 'jc'.
    Je,                               // Instruction 'je'.
    Jecxz,                            // Instruction 'jecxz'.
    Jg,                               // Instruction 'jg'.
    Jge,                              // Instruction 'jge'.
    Jl,                               // Instruction 'jl'.
    Jle,                              // Instruction 'jle'.
    Jmp,                              // Instruction 'jmp'.
    Jna,                              // Instruction 'jna'.
    Jnae,                             // Instruction 'jnae'.
    Jnb,                              // Instruction 'jnb'.
    Jnbe,                             // Instruction 'jnbe'.
    Jnc,                              // Instruction 'jnc'.
    Jne,                              // Instruction 'jne'.
    Jng,                              // Instruction 'jng'.
    Jnge,                             // Instruction 'jnge'.
    Jnl,                              // Instruction 'jnl'.
    Jnle,                             // Instruction 'jnle'.
    Jno,                              // Instruction 'jno'.
    Jnp,                              // Instruction 'jnp'.
    Jns,                              // Instruction 'jns'.
    Jnz,                              // Instruction 'jnz'.
    Jo,                               // Instruction 'jo'.
    Jp,                               // Instruction 'jp'.
    Jpe,                              // Instruction 'jpe'.
    Jpo,                              // Instruction 'jpo'.
    Js,                               // Instruction 'js'.
    Jz,                               // Instruction 'jz'.
    Kaddb,                            // Instruction 'kaddb' {AVX512_DQ}.
    Kaddd,                            // Instruction 'kaddd' {AVX512_BW}.
    Kaddq,                            // Instruction 'kaddq' {AVX512_BW}.
    Kaddw,                            // Instruction 'kaddw' {AVX512_DQ}.
    Kandb,                            // Instruction 'kandb' {AVX512_DQ}.
    Kandd,                            // Instruction 'kandd' {AVX512_BW}.
    Kandnb,                           // Instruction 'kandnb' {AVX512_DQ}.
    Kandnd,                           // Instruction 'kandnd' {AVX512_BW}.
    Kandnq,                           // Instruction 'kandnq' {AVX512_BW}.
    Kandnw,                           // Instruction 'kandnw' {AVX512_F}.
    Kandq,                            // Instruction 'kandq' {AVX512_BW}.
    Kandw,                            // Instruction 'kandw' {AVX512_F}.
    Kmovb,                            // Instruction 'kmovb' {AVX512_DQ}.
    Kmovd,                            // Instruction 'kmovd' {AVX512_BW}.
    Kmovq,                            // Instruction 'kmovq' {AVX512_BW}.
    Kmovw,                            // Instruction 'kmovw' {AVX512_F}.
    Knotb,                            // Instruction 'knotb' {AVX512_DQ}.
    Knotd,                            // Instruction 'knotd' {AVX512_BW}.
    Knotq,                            // Instruction 'knotq' {AVX512_BW}.
    Knotw,                            // Instruction 'knotw' {AVX512_F}.
    Korb,                             // Instruction 'korb' {AVX512_DQ}.
    Kord,                             // Instruction 'kord' {AVX512_BW}.
    Korq,                             // Instruction 'korq' {AVX512_BW}.
    Kortestb,                         // Instruction 'kortestb' {AVX512_DQ}.
    Kortestd,                         // Instruction 'kortestd' {AVX512_BW}.
    Kortestq,                         // Instruction 'kortestq' {AVX512_BW}.
    Kortestw,                         // Instruction 'kortestw' {AVX512_F}.
    Korw,                             // Instruction 'korw' {AVX512_F}.
    Kshiftlb,                         // Instruction 'kshiftlb' {AVX512_DQ}.
    Kshiftld,                         // Instruction 'kshiftld' {AVX512_BW}.
    Kshiftlq,                         // Instruction 'kshiftlq' {AVX512_BW}.
    Kshiftlw,                         // Instruction 'kshiftlw' {AVX512_F}.
    Kshiftrb,                         // Instruction 'kshiftrb' {AVX512_DQ}.
    Kshiftrd,                         // Instruction 'kshiftrd' {AVX512_BW}.
    Kshiftrq,                         // Instruction 'kshiftrq' {AVX512_BW}.
    Kshiftrw,                         // Instruction 'kshiftrw' {AVX512_F}.
    Ktestb,                           // Instruction 'ktestb' {AVX512_DQ}.
    Ktestd,                           // Instruction 'ktestd' {AVX512_BW}.
    Ktestq,                           // Instruction 'ktestq' {AVX512_BW}.
    Ktestw,                           // Instruction 'ktestw' {AVX512_DQ}.
    Kunpckbw,                         // Instruction 'kunpckbw' {AVX512_F}.
    Kunpckdq,                         // Instruction 'kunpckdq' {AVX512_BW}.
    Kunpckwd,                         // Instruction 'kunpckwd' {AVX512_BW}.
    Kxnorb,                           // Instruction 'kxnorb' {AVX512_DQ}.
    Kxnord,                           // Instruction 'kxnord' {AVX512_BW}.
    Kxnorq,                           // Instruction 'kxnorq' {AVX512_BW}.
    Kxnorw,                           // Instruction 'kxnorw' {AVX512_F}.
    Kxorb,                            // Instruction 'kxorb' {AVX512_DQ}.
    Kxord,                            // Instruction 'kxord' {AVX512_BW}.
    Kxorq,                            // Instruction 'kxorq' {AVX512_BW}.
    Kxorw,                            // Instruction 'kxorw' {AVX512_F}.
    Lahf,                             // Instruction 'lahf' {LAHFSAHF}.
    Lar,                              // Instruction 'lar'.
    Lddqu,                            // Instruction 'lddqu' {SSE3}.
    Ldmxcsr,                          // Instruction 'ldmxcsr' {SSE}.
    Lds,                              // Instruction 'lds' (X86).
    Lea,                              // Instruction 'lea'.
    Leave,                            // Instruction 'leave'.
    Les,                              // Instruction 'les' (X86).
    Lfence,                           // Instruction 'lfence' {SSE2}.
    Lfs,                              // Instruction 'lfs'.
    Lgdt,                             // Instruction 'lgdt'.
    Lgs,                              // Instruction 'lgs'.
    Lidt,                             // Instruction 'lidt'.
    Lldt,                             // Instruction 'lldt'.
    Llwpcb,                           // Instruction 'llwpcb' {LWP}.
    Lmsw,                             // Instruction 'lmsw'.
    Lods,                             // Instruction 'lods'.
    Loop,                             // Instruction 'loop'.
    Loope,                            // Instruction 'loope'.
    Loopne,                           // Instruction 'loopne'.
    Lsl,                              // Instruction 'lsl'.
    Lss,                              // Instruction 'lss'.
    Ltr,                              // Instruction 'ltr'.
    Lwpins,                           // Instruction 'lwpins' {LWP}.
    Lwpval,                           // Instruction 'lwpval' {LWP}.
    Lzcnt,                            // Instruction 'lzcnt' {LZCNT}.
    Maskmovdqu,                       // Instruction 'maskmovdqu' {SSE2}.
    Maskmovq,                         // Instruction 'maskmovq' {MMX2}.
    Maxpd,                            // Instruction 'maxpd' {SSE2}.
    Maxps,                            // Instruction 'maxps' {SSE}.
    Maxsd,                            // Instruction 'maxsd' {SSE2}.
    Maxss,                            // Instruction 'maxss' {SSE}.
    Mfence,                           // Instruction 'mfence' {SSE2}.
    Minpd,                            // Instruction 'minpd' {SSE2}.
    Minps,                            // Instruction 'minps' {SSE}.
    Minsd,                            // Instruction 'minsd' {SSE2}.
    Minss,                            // Instruction 'minss' {SSE}.
    Monitor,                          // Instruction 'monitor' {MONITOR}.
    Monitorx,                         // Instruction 'monitorx' {MONITORX}.
    Mov,                              // Instruction 'mov'.
    Movapd,                           // Instruction 'movapd' {SSE2}.
    Movaps,                           // Instruction 'movaps' {SSE}.
    Movbe,                            // Instruction 'movbe' {MOVBE}.
    Movd,                             // Instruction 'movd' {MMX|SSE2}.
    Movddup,                          // Instruction 'movddup' {SSE3}.
    Movdir64b,                        // Instruction 'movdir64b' {MOVDIR64B}.
    Movdiri,                          // Instruction 'movdiri' {MOVDIRI}.
    Movdq2q,                          // Instruction 'movdq2q' {SSE2}.
    Movdqa,                           // Instruction 'movdqa' {SSE2}.
    Movdqu,                           // Instruction 'movdqu' {SSE2}.
    Movhlps,                          // Instruction 'movhlps' {SSE}.
    Movhpd,                           // Instruction 'movhpd' {SSE2}.
    Movhps,                           // Instruction 'movhps' {SSE}.
    Movlhps,                          // Instruction 'movlhps' {SSE}.
    Movlpd,                           // Instruction 'movlpd' {SSE2}.
    Movlps,                           // Instruction 'movlps' {SSE}.
    Movmskpd,                         // Instruction 'movmskpd' {SSE2}.
    Movmskps,                         // Instruction 'movmskps' {SSE}.
    Movntdq,                          // Instruction 'movntdq' {SSE2}.
    Movntdqa,                         // Instruction 'movntdqa' {SSE4_1}.
    Movnti,                           // Instruction 'movnti' {SSE2}.
    Movntpd,                          // Instruction 'movntpd' {SSE2}.
    Movntps,                          // Instruction 'movntps' {SSE}.
    Movntq,                           // Instruction 'movntq' {MMX2}.
    Movntsd,                          // Instruction 'movntsd' {SSE4A}.
    Movntss,                          // Instruction 'movntss' {SSE4A}.
    Movq,                             // Instruction 'movq' {MMX|SSE2}.
    Movq2dq,                          // Instruction 'movq2dq' {SSE2}.
    Movs,                             // Instruction 'movs'.
    Movsd,                            // Instruction 'movsd' {SSE2}.
    Movshdup,                         // Instruction 'movshdup' {SSE3}.
    Movsldup,                         // Instruction 'movsldup' {SSE3}.
    Movss,                            // Instruction 'movss' {SSE}.
    Movsx,                            // Instruction 'movsx'.
    Movsxd,                           // Instruction 'movsxd' (X64).
    Movupd,                           // Instruction 'movupd' {SSE2}.
    Movups,                           // Instruction 'movups' {SSE}.
    Movzx,                            // Instruction 'movzx'.
    Mpsadbw,                          // Instruction 'mpsadbw' {SSE4_1}.
    Mul,                              // Instruction 'mul'.
    Mulpd,                            // Instruction 'mulpd' {SSE2}.
    Mulps,                            // Instruction 'mulps' {SSE}.
    Mulsd,                            // Instruction 'mulsd' {SSE2}.
    Mulss,                            // Instruction 'mulss' {SSE}.
    Mulx,                             // Instruction 'mulx' {BMI2}.
    Mwait,                            // Instruction 'mwait' {MONITOR}.
    Mwaitx,                           // Instruction 'mwaitx' {MONITORX}.
    Neg,                              // Instruction 'neg'.
    Nop,                              // Instruction 'nop'.
    Not,                              // Instruction 'not'.
    Or,                               // Instruction 'or'.
    Orpd,                             // Instruction 'orpd' {SSE2}.
    Orps,                             // Instruction 'orps' {SSE}.
    Out,                              // Instruction 'out'.
    Outs,                             // Instruction 'outs'.
    Pabsb,                            // Instruction 'pabsb' {SSSE3}.
    Pabsd,                            // Instruction 'pabsd' {SSSE3}.
    Pabsw,                            // Instruction 'pabsw' {SSSE3}.
    Packssdw,                         // Instruction 'packssdw' {MMX|SSE2}.
    Packsswb,                         // Instruction 'packsswb' {MMX|SSE2}.
    Packusdw,                         // Instruction 'packusdw' {SSE4_1}.
    Packuswb,                         // Instruction 'packuswb' {MMX|SSE2}.
    Paddb,                            // Instruction 'paddb' {MMX|SSE2}.
    Paddd,                            // Instruction 'paddd' {MMX|SSE2}.
    Paddq,                            // Instruction 'paddq' {SSE2}.
    Paddsb,                           // Instruction 'paddsb' {MMX|SSE2}.
    Paddsw,                           // Instruction 'paddsw' {MMX|SSE2}.
    Paddusb,                          // Instruction 'paddusb' {MMX|SSE2}.
    Paddusw,                          // Instruction 'paddusw' {MMX|SSE2}.
    Paddw,                            // Instruction 'paddw' {MMX|SSE2}.
    Palignr,                          // Instruction 'palignr' {SSE3}.
    Pand,                             // Instruction 'pand' {MMX|SSE2}.
    Pandn,                            // Instruction 'pandn' {MMX|SSE2}.
    Pause,                            // Instruction 'pause'.
    Pavgb,                            // Instruction 'pavgb' {MMX2|SSE2}.
    Pavgusb,                          // Instruction 'pavgusb' {3DNOW}.
    Pavgw,                            // Instruction 'pavgw' {MMX2|SSE2}.
    Pblendvb,                         // Instruction 'pblendvb' {SSE4_1}.
    Pblendw,                          // Instruction 'pblendw' {SSE4_1}.
    Pclmulqdq,                        // Instruction 'pclmulqdq' {PCLMULQDQ}.
    Pcmpeqb,                          // Instruction 'pcmpeqb' {MMX|SSE2}.
    Pcmpeqd,                          // Instruction 'pcmpeqd' {MMX|SSE2}.
    Pcmpeqq,                          // Instruction 'pcmpeqq' {SSE4_1}.
    Pcmpeqw,                          // Instruction 'pcmpeqw' {MMX|SSE2}.
    Pcmpestri,                        // Instruction 'pcmpestri' {SSE4_2}.
    Pcmpestrm,                        // Instruction 'pcmpestrm' {SSE4_2}.
    Pcmpgtb,                          // Instruction 'pcmpgtb' {MMX|SSE2}.
    Pcmpgtd,                          // Instruction 'pcmpgtd' {MMX|SSE2}.
    Pcmpgtq,                          // Instruction 'pcmpgtq' {SSE4_2}.
    Pcmpgtw,                          // Instruction 'pcmpgtw' {MMX|SSE2}.
    Pcmpistri,                        // Instruction 'pcmpistri' {SSE4_2}.
    Pcmpistrm,                        // Instruction 'pcmpistrm' {SSE4_2}.
    Pcommit,                          // Instruction 'pcommit' {PCOMMIT}.
    Pdep,                             // Instruction 'pdep' {BMI2}.
    Pext,                             // Instruction 'pext' {BMI2}.
    Pextrb,                           // Instruction 'pextrb' {SSE4_1}.
    Pextrd,                           // Instruction 'pextrd' {SSE4_1}.
    Pextrq,                           // Instruction 'pextrq' {SSE4_1} (X64).
    Pextrw,                           // Instruction 'pextrw' {MMX2|SSE2|SSE4_1}.
    Pf2id,                            // Instruction 'pf2id' {3DNOW}.
    Pf2iw,                            // Instruction 'pf2iw' {3DNOW2}.
    Pfacc,                            // Instruction 'pfacc' {3DNOW}.
    Pfadd,                            // Instruction 'pfadd' {3DNOW}.
    Pfcmpeq,                          // Instruction 'pfcmpeq' {3DNOW}.
    Pfcmpge,                          // Instruction 'pfcmpge' {3DNOW}.
    Pfcmpgt,                          // Instruction 'pfcmpgt' {3DNOW}.
    Pfmax,                            // Instruction 'pfmax' {3DNOW}.
    Pfmin,                            // Instruction 'pfmin' {3DNOW}.
    Pfmul,                            // Instruction 'pfmul' {3DNOW}.
    Pfnacc,                           // Instruction 'pfnacc' {3DNOW2}.
    Pfpnacc,                          // Instruction 'pfpnacc' {3DNOW2}.
    Pfrcp,                            // Instruction 'pfrcp' {3DNOW}.
    Pfrcpit1,                         // Instruction 'pfrcpit1' {3DNOW}.
    Pfrcpit2,                         // Instruction 'pfrcpit2' {3DNOW}.
    Pfrcpv,                           // Instruction 'pfrcpv' {GEODE}.
    Pfrsqit1,                         // Instruction 'pfrsqit1' {3DNOW}.
    Pfrsqrt,                          // Instruction 'pfrsqrt' {3DNOW}.
    Pfrsqrtv,                         // Instruction 'pfrsqrtv' {GEODE}.
    Pfsub,                            // Instruction 'pfsub' {3DNOW}.
    Pfsubr,                           // Instruction 'pfsubr' {3DNOW}.
    Phaddd,                           // Instruction 'phaddd' {SSSE3}.
    Phaddsw,                          // Instruction 'phaddsw' {SSSE3}.
    Phaddw,                           // Instruction 'phaddw' {SSSE3}.
    Phminposuw,                       // Instruction 'phminposuw' {SSE4_1}.
    Phsubd,                           // Instruction 'phsubd' {SSSE3}.
    Phsubsw,                          // Instruction 'phsubsw' {SSSE3}.
    Phsubw,                           // Instruction 'phsubw' {SSSE3}.
    Pi2fd,                            // Instruction 'pi2fd' {3DNOW}.
    Pi2fw,                            // Instruction 'pi2fw' {3DNOW2}.
    Pinsrb,                           // Instruction 'pinsrb' {SSE4_1}.
    Pinsrd,                           // Instruction 'pinsrd' {SSE4_1}.
    Pinsrq,                           // Instruction 'pinsrq' {SSE4_1} (X64).
    Pinsrw,                           // Instruction 'pinsrw' {MMX2|SSE2}.
    Pmaddubsw,                        // Instruction 'pmaddubsw' {SSSE3}.
    Pmaddwd,                          // Instruction 'pmaddwd' {MMX|SSE2}.
    Pmaxsb,                           // Instruction 'pmaxsb' {SSE4_1}.
    Pmaxsd,                           // Instruction 'pmaxsd' {SSE4_1}.
    Pmaxsw,                           // Instruction 'pmaxsw' {MMX2|SSE2}.
    Pmaxub,                           // Instruction 'pmaxub' {MMX2|SSE2}.
    Pmaxud,                           // Instruction 'pmaxud' {SSE4_1}.
    Pmaxuw,                           // Instruction 'pmaxuw' {SSE4_1}.
    Pminsb,                           // Instruction 'pminsb' {SSE4_1}.
    Pminsd,                           // Instruction 'pminsd' {SSE4_1}.
    Pminsw,                           // Instruction 'pminsw' {MMX2|SSE2}.
    Pminub,                           // Instruction 'pminub' {MMX2|SSE2}.
    Pminud,                           // Instruction 'pminud' {SSE4_1}.
    Pminuw,                           // Instruction 'pminuw' {SSE4_1}.
    Pmovmskb,                         // Instruction 'pmovmskb' {MMX2|SSE2}.
    Pmovsxbd,                         // Instruction 'pmovsxbd' {SSE4_1}.
    Pmovsxbq,                         // Instruction 'pmovsxbq' {SSE4_1}.
    Pmovsxbw,                         // Instruction 'pmovsxbw' {SSE4_1}.
    Pmovsxdq,                         // Instruction 'pmovsxdq' {SSE4_1}.
    Pmovsxwd,                         // Instruction 'pmovsxwd' {SSE4_1}.
    Pmovsxwq,                         // Instruction 'pmovsxwq' {SSE4_1}.
    Pmovzxbd,                         // Instruction 'pmovzxbd' {SSE4_1}.
    Pmovzxbq,                         // Instruction 'pmovzxbq' {SSE4_1}.
    Pmovzxbw,                         // Instruction 'pmovzxbw' {SSE4_1}.
    Pmovzxdq,                         // Instruction 'pmovzxdq' {SSE4_1}.
    Pmovzxwd,                         // Instruction 'pmovzxwd' {SSE4_1}.
    Pmovzxwq,                         // Instruction 'pmovzxwq' {SSE4_1}.
    Pmuldq,                           // Instruction 'pmuldq' {SSE4_1}.
    Pmulhrsw,                         // Instruction 'pmulhrsw' {SSSE3}.
    Pmulhrw,                          // Instruction 'pmulhrw' {3DNOW}.
    Pmulhuw,                          // Instruction 'pmulhuw' {MMX2|SSE2}.
    Pmulhw,                           // Instruction 'pmulhw' {MMX|SSE2}.
    Pmulld,                           // Instruction 'pmulld' {SSE4_1}.
    Pmullw,                           // Instruction 'pmullw' {MMX|SSE2}.
    Pmuludq,                          // Instruction 'pmuludq' {SSE2}.
    Pop,                              // Instruction 'pop'.
    Popa,                             // Instruction 'popa' (X86).
    Popad,                            // Instruction 'popad' (X86).
    Popcnt,                           // Instruction 'popcnt' {POPCNT}.
    Popf,                             // Instruction 'popf'.
    Popfd,                            // Instruction 'popfd' (X86).
    Popfq,                            // Instruction 'popfq' (X64).
    Por,                              // Instruction 'por' {MMX|SSE2}.
    Prefetch,                         // Instruction 'prefetch' {3DNOW}.
    Prefetchnta,                      // Instruction 'prefetchnta' {MMX2}.
    Prefetcht0,                       // Instruction 'prefetcht0' {MMX2}.
    Prefetcht1,                       // Instruction 'prefetcht1' {MMX2}.
    Prefetcht2,                       // Instruction 'prefetcht2' {MMX2}.
    Prefetchw,                        // Instruction 'prefetchw' {PREFETCHW}.
    Prefetchwt1,                      // Instruction 'prefetchwt1' {PREFETCHWT1}.
    Psadbw,                           // Instruction 'psadbw' {MMX2|SSE2}.
    Pshufb,                           // Instruction 'pshufb' {SSSE3}.
    Pshufd,                           // Instruction 'pshufd' {SSE2}.
    Pshufhw,                          // Instruction 'pshufhw' {SSE2}.
    Pshuflw,                          // Instruction 'pshuflw' {SSE2}.
    Pshufw,                           // Instruction 'pshufw' {MMX2}.
    Psignb,                           // Instruction 'psignb' {SSSE3}.
    Psignd,                           // Instruction 'psignd' {SSSE3}.
    Psignw,                           // Instruction 'psignw' {SSSE3}.
    Pslld,                            // Instruction 'pslld' {MMX|SSE2}.
    Pslldq,                           // Instruction 'pslldq' {SSE2}.
    Psllq,                            // Instruction 'psllq' {MMX|SSE2}.
    Psllw,                            // Instruction 'psllw' {MMX|SSE2}.
    Psrad,                            // Instruction 'psrad' {MMX|SSE2}.
    Psraw,                            // Instruction 'psraw' {MMX|SSE2}.
    Psrld,                            // Instruction 'psrld' {MMX|SSE2}.
    Psrldq,                           // Instruction 'psrldq' {SSE2}.
    Psrlq,                            // Instruction 'psrlq' {MMX|SSE2}.
    Psrlw,                            // Instruction 'psrlw' {MMX|SSE2}.
    Psubb,                            // Instruction 'psubb' {MMX|SSE2}.
    Psubd,                            // Instruction 'psubd' {MMX|SSE2}.
    Psubq,                            // Instruction 'psubq' {SSE2}.
    Psubsb,                           // Instruction 'psubsb' {MMX|SSE2}.
    Psubsw,                           // Instruction 'psubsw' {MMX|SSE2}.
    Psubusb,                          // Instruction 'psubusb' {MMX|SSE2}.
    Psubusw,                          // Instruction 'psubusw' {MMX|SSE2}.
    Psubw,                            // Instruction 'psubw' {MMX|SSE2}.
    Pswapd,                           // Instruction 'pswapd' {3DNOW2}.
    Ptest,                            // Instruction 'ptest' {SSE4_1}.
    Punpckhbw,                        // Instruction 'punpckhbw' {MMX|SSE2}.
    Punpckhdq,                        // Instruction 'punpckhdq' {MMX|SSE2}.
    Punpckhqdq,                       // Instruction 'punpckhqdq' {SSE2}.
    Punpckhwd,                        // Instruction 'punpckhwd' {MMX|SSE2}.
    Punpcklbw,                        // Instruction 'punpcklbw' {MMX|SSE2}.
    Punpckldq,                        // Instruction 'punpckldq' {MMX|SSE2}.
    Punpcklqdq,                       // Instruction 'punpcklqdq' {SSE2}.
    Punpcklwd,                        // Instruction 'punpcklwd' {MMX|SSE2}.
    Push,                             // Instruction 'push'.
    Pusha,                            // Instruction 'pusha' (X86).
    Pushad,                           // Instruction 'pushad' (X86).
    Pushf,                            // Instruction 'pushf'.
    Pushfd,                           // Instruction 'pushfd' (X86).
    Pushfq,                           // Instruction 'pushfq' (X64).
    Pxor,                             // Instruction 'pxor' {MMX|SSE2}.
    Rcl,                              // Instruction 'rcl'.
    Rcpps,                            // Instruction 'rcpps' {SSE}.
    Rcpss,                            // Instruction 'rcpss' {SSE}.
    Rcr,                              // Instruction 'rcr'.
    Rdfsbase,                         // Instruction 'rdfsbase' {FSGSBASE} (X64).
    Rdgsbase,                         // Instruction 'rdgsbase' {FSGSBASE} (X64).
    Rdmsr,                            // Instruction 'rdmsr' {MSR}.
    Rdpid,                            // Instruction 'rdpid' {RDPID}.
    Rdpmc,                            // Instruction 'rdpmc'.
    Rdrand,                           // Instruction 'rdrand' {RDRAND}.
    Rdseed,                           // Instruction 'rdseed' {RDSEED}.
    Rdtsc,                            // Instruction 'rdtsc' {RDTSC}.
    Rdtscp,                           // Instruction 'rdtscp' {RDTSCP}.
    Ret,                              // Instruction 'ret'.
    Rol,                              // Instruction 'rol'.
    Ror,                              // Instruction 'ror'.
    Rorx,                             // Instruction 'rorx' {BMI2}.
    Roundpd,                          // Instruction 'roundpd' {SSE4_1}.
    Roundps,                          // Instruction 'roundps' {SSE4_1}.
    Roundsd,                          // Instruction 'roundsd' {SSE4_1}.
    Roundss,                          // Instruction 'roundss' {SSE4_1}.
    Rsm,                              // Instruction 'rsm' (X86).
    Rsqrtps,                          // Instruction 'rsqrtps' {SSE}.
    Rsqrtss,                          // Instruction 'rsqrtss' {SSE}.
    Sahf,                             // Instruction 'sahf' {LAHFSAHF}.
    Sal,                              // Instruction 'sal'.
    Sar,                              // Instruction 'sar'.
    Sarx,                             // Instruction 'sarx' {BMI2}.
    Sbb,                              // Instruction 'sbb'.
    Scas,                             // Instruction 'scas'.
    Seta,                             // Instruction 'seta'.
    Setae,                            // Instruction 'setae'.
    Setb,                             // Instruction 'setb'.
    Setbe,                            // Instruction 'setbe'.
    Setc,                             // Instruction 'setc'.
    Sete,                             // Instruction 'sete'.
    Setg,                             // Instruction 'setg'.
    Setge,                            // Instruction 'setge'.
    Setl,                             // Instruction 'setl'.
    Setle,                            // Instruction 'setle'.
    Setna,                            // Instruction 'setna'.
    Setnae,                           // Instruction 'setnae'.
    Setnb,                            // Instruction 'setnb'.
    Setnbe,                           // Instruction 'setnbe'.
    Setnc,                            // Instruction 'setnc'.
    Setne,                            // Instruction 'setne'.
    Setng,                            // Instruction 'setng'.
    Setnge,                           // Instruction 'setnge'.
    Setnl,                            // Instruction 'setnl'.
    Setnle,                           // Instruction 'setnle'.
    Setno,                            // Instruction 'setno'.
    Setnp,                            // Instruction 'setnp'.
    Setns,                            // Instruction 'setns'.
    Setnz,                            // Instruction 'setnz'.
    Seto,                             // Instruction 'seto'.
    Setp,                             // Instruction 'setp'.
    Setpe,                            // Instruction 'setpe'.
    Setpo,                            // Instruction 'setpo'.
    Sets,                             // Instruction 'sets'.
    Setz,                             // Instruction 'setz'.
    Sfence,                           // Instruction 'sfence' {MMX2}.
    Sgdt,                             // Instruction 'sgdt'.
    Sha1msg1,                         // Instruction 'sha1msg1' {SHA}.
    Sha1msg2,                         // Instruction 'sha1msg2' {SHA}.
    Sha1nexte,                        // Instruction 'sha1nexte' {SHA}.
    Sha1rnds4,                        // Instruction 'sha1rnds4' {SHA}.
    Sha256msg1,                       // Instruction 'sha256msg1' {SHA}.
    Sha256msg2,                       // Instruction 'sha256msg2' {SHA}.
    Sha256rnds2,                      // Instruction 'sha256rnds2' {SHA}.
    Shl,                              // Instruction 'shl'.
    Shld,                             // Instruction 'shld'.
    Shlx,                             // Instruction 'shlx' {BMI2}.
    Shr,                              // Instruction 'shr'.
    Shrd,                             // Instruction 'shrd'.
    Shrx,                             // Instruction 'shrx' {BMI2}.
    Shufpd,                           // Instruction 'shufpd' {SSE2}.
    Shufps,                           // Instruction 'shufps' {SSE}.
    Sidt,                             // Instruction 'sidt'.
    Skinit,                           // Instruction 'skinit' {SKINIT}.
    Sldt,                             // Instruction 'sldt'.
    Slwpcb,                           // Instruction 'slwpcb' {LWP}.
    Smsw,                             // Instruction 'smsw'.
    Sqrtpd,                           // Instruction 'sqrtpd' {SSE2}.
    Sqrtps,                           // Instruction 'sqrtps' {SSE}.
    Sqrtsd,                           // Instruction 'sqrtsd' {SSE2}.
    Sqrtss,                           // Instruction 'sqrtss' {SSE}.
    Stac,                             // Instruction 'stac' {SMAP}.
    Stc,                              // Instruction 'stc'.
    Std,                              // Instruction 'std'.
    Stgi,                             // Instruction 'stgi' {SKINIT}.
    Sti,                              // Instruction 'sti'.
    Stmxcsr,                          // Instruction 'stmxcsr' {SSE}.
    Stos,                             // Instruction 'stos'.
    Str,                              // Instruction 'str'.
    Sub,                              // Instruction 'sub'.
    Subpd,                            // Instruction 'subpd' {SSE2}.
    Subps,                            // Instruction 'subps' {SSE}.
    Subsd,                            // Instruction 'subsd' {SSE2}.
    Subss,                            // Instruction 'subss' {SSE}.
    Swapgs,                           // Instruction 'swapgs' (X64).
    Syscall,                          // Instruction 'syscall' (X64).
    Sysenter,                         // Instruction 'sysenter'.
    Sysexit,                          // Instruction 'sysexit'.
    Sysexit64,                        // Instruction 'sysexit64'.
    Sysret,                           // Instruction 'sysret' (X64).
    Sysret64,                         // Instruction 'sysret64' (X64).
    T1mskc,                           // Instruction 't1mskc' {TBM}.
    Test,                             // Instruction 'test'.
    Tzcnt,                            // Instruction 'tzcnt' {BMI}.
    Tzmsk,                            // Instruction 'tzmsk' {TBM}.
    Ucomisd,                          // Instruction 'ucomisd' {SSE2}.
    Ucomiss,                          // Instruction 'ucomiss' {SSE}.
    Ud2,                              // Instruction 'ud2'.
    Unpckhpd,                         // Instruction 'unpckhpd' {SSE2}.
    Unpckhps,                         // Instruction 'unpckhps' {SSE}.
    Unpcklpd,                         // Instruction 'unpcklpd' {SSE2}.
    Unpcklps,                         // Instruction 'unpcklps' {SSE}.
    V4fmaddps,                        // Instruction 'v4fmaddps' {AVX512_4FMAPS}.
    V4fmaddss,                        // Instruction 'v4fmaddss' {AVX512_4FMAPS}.
    V4fnmaddps,                       // Instruction 'v4fnmaddps' {AVX512_4FMAPS}.
    V4fnmaddss,                       // Instruction 'v4fnmaddss' {AVX512_4FMAPS}.
    Vaddpd,                           // Instruction 'vaddpd' {AVX|AVX512_F+VL}.
    Vaddps,                           // Instruction 'vaddps' {AVX|AVX512_F+VL}.
    Vaddsd,                           // Instruction 'vaddsd' {AVX|AVX512_F}.
    Vaddss,                           // Instruction 'vaddss' {AVX|AVX512_F}.
    Vaddsubpd,                        // Instruction 'vaddsubpd' {AVX}.
    Vaddsubps,                        // Instruction 'vaddsubps' {AVX}.
    Vaesdec,                          // Instruction 'vaesdec' {AVX|AVX512_F+VL & AESNI|VAES}.
    Vaesdeclast,                      // Instruction 'vaesdeclast' {AVX|AVX512_F+VL & AESNI|VAES}.
    Vaesenc,                          // Instruction 'vaesenc' {AVX|AVX512_F+VL & AESNI|VAES}.
    Vaesenclast,                      // Instruction 'vaesenclast' {AVX|AVX512_F+VL & AESNI|VAES}.
    Vaesimc,                          // Instruction 'vaesimc' {AVX & AESNI}.
    Vaeskeygenassist,                 // Instruction 'vaeskeygenassist' {AVX & AESNI}.
    Valignd,                          // Instruction 'valignd' {AVX512_F+VL}.
    Valignq,                          // Instruction 'valignq' {AVX512_F+VL}.
    Vandnpd,                          // Instruction 'vandnpd' {AVX|AVX512_DQ+VL}.
    Vandnps,                          // Instruction 'vandnps' {AVX|AVX512_DQ+VL}.
    Vandpd,                           // Instruction 'vandpd' {AVX|AVX512_DQ+VL}.
    Vandps,                           // Instruction 'vandps' {AVX|AVX512_DQ+VL}.
    Vblendmb,                         // Instruction 'vblendmb' {AVX512_BW+VL}.
    Vblendmd,                         // Instruction 'vblendmd' {AVX512_F+VL}.
    Vblendmpd,                        // Instruction 'vblendmpd' {AVX512_F+VL}.
    Vblendmps,                        // Instruction 'vblendmps' {AVX512_F+VL}.
    Vblendmq,                         // Instruction 'vblendmq' {AVX512_F+VL}.
    Vblendmw,                         // Instruction 'vblendmw' {AVX512_BW+VL}.
    Vblendpd,                         // Instruction 'vblendpd' {AVX}.
    Vblendps,                         // Instruction 'vblendps' {AVX}.
    Vblendvpd,                        // Instruction 'vblendvpd' {AVX}.
    Vblendvps,                        // Instruction 'vblendvps' {AVX}.
    Vbroadcastf128,                   // Instruction 'vbroadcastf128' {AVX}.
    Vbroadcastf32x2,                  // Instruction 'vbroadcastf32x2' {AVX512_DQ+VL}.
    Vbroadcastf32x4,                  // Instruction 'vbroadcastf32x4' {AVX512_F}.
    Vbroadcastf32x8,                  // Instruction 'vbroadcastf32x8' {AVX512_DQ}.
    Vbroadcastf64x2,                  // Instruction 'vbroadcastf64x2' {AVX512_DQ+VL}.
    Vbroadcastf64x4,                  // Instruction 'vbroadcastf64x4' {AVX512_F}.
    Vbroadcasti128,                   // Instruction 'vbroadcasti128' {AVX2}.
    Vbroadcasti32x2,                  // Instruction 'vbroadcasti32x2' {AVX512_DQ+VL}.
    Vbroadcasti32x4,                  // Instruction 'vbroadcasti32x4' {AVX512_F+VL}.
    Vbroadcasti32x8,                  // Instruction 'vbroadcasti32x8' {AVX512_DQ}.
    Vbroadcasti64x2,                  // Instruction 'vbroadcasti64x2' {AVX512_DQ+VL}.
    Vbroadcasti64x4,                  // Instruction 'vbroadcasti64x4' {AVX512_F}.
    Vbroadcastsd,                     // Instruction 'vbroadcastsd' {AVX|AVX2|AVX512_F+VL}.
    Vbroadcastss,                     // Instruction 'vbroadcastss' {AVX|AVX2|AVX512_F+VL}.
    Vcmppd,                           // Instruction 'vcmppd' {AVX|AVX512_F+VL}.
    Vcmpps,                           // Instruction 'vcmpps' {AVX|AVX512_F+VL}.
    Vcmpsd,                           // Instruction 'vcmpsd' {AVX|AVX512_F}.
    Vcmpss,                           // Instruction 'vcmpss' {AVX|AVX512_F}.
    Vcomisd,                          // Instruction 'vcomisd' {AVX|AVX512_F}.
    Vcomiss,                          // Instruction 'vcomiss' {AVX|AVX512_F}.
    Vcompresspd,                      // Instruction 'vcompresspd' {AVX512_F+VL}.
    Vcompressps,                      // Instruction 'vcompressps' {AVX512_F+VL}.
    Vcvtdq2pd,                        // Instruction 'vcvtdq2pd' {AVX|AVX512_F+VL}.
    Vcvtdq2ps,                        // Instruction 'vcvtdq2ps' {AVX|AVX512_F+VL}.
    Vcvtne2ps2bf16,                   // Instruction 'vcvtne2ps2bf16' {AVX512_BF16+VL}.
    Vcvtneps2bf16,                    // Instruction 'vcvtneps2bf16' {AVX512_BF16+VL}.
    Vcvtpd2dq,                        // Instruction 'vcvtpd2dq' {AVX|AVX512_F+VL}.
    Vcvtpd2ps,                        // Instruction 'vcvtpd2ps' {AVX|AVX512_F+VL}.
    Vcvtpd2qq,                        // Instruction 'vcvtpd2qq' {AVX512_DQ+VL}.
    Vcvtpd2udq,                       // Instruction 'vcvtpd2udq' {AVX512_F+VL}.
    Vcvtpd2uqq,                       // Instruction 'vcvtpd2uqq' {AVX512_DQ+VL}.
    Vcvtph2ps,                        // Instruction 'vcvtph2ps' {AVX512_F+VL & F16C}.
    Vcvtps2dq,                        // Instruction 'vcvtps2dq' {AVX|AVX512_F+VL}.
    Vcvtps2pd,                        // Instruction 'vcvtps2pd' {AVX|AVX512_F+VL}.
    Vcvtps2ph,                        // Instruction 'vcvtps2ph' {AVX512_F+VL & F16C}.
    Vcvtps2qq,                        // Instruction 'vcvtps2qq' {AVX512_DQ+VL}.
    Vcvtps2udq,                       // Instruction 'vcvtps2udq' {AVX512_F+VL}.
    Vcvtps2uqq,                       // Instruction 'vcvtps2uqq' {AVX512_DQ+VL}.
    Vcvtqq2pd,                        // Instruction 'vcvtqq2pd' {AVX512_DQ+VL}.
    Vcvtqq2ps,                        // Instruction 'vcvtqq2ps' {AVX512_DQ+VL}.
    Vcvtsd2si,                        // Instruction 'vcvtsd2si' {AVX|AVX512_F}.
    Vcvtsd2ss,                        // Instruction 'vcvtsd2ss' {AVX|AVX512_F}.
    Vcvtsd2usi,                       // Instruction 'vcvtsd2usi' {AVX512_F}.
    Vcvtsi2sd,                        // Instruction 'vcvtsi2sd' {AVX|AVX512_F}.
    Vcvtsi2ss,                        // Instruction 'vcvtsi2ss' {AVX|AVX512_F}.
    Vcvtss2sd,                        // Instruction 'vcvtss2sd' {AVX|AVX512_F}.
    Vcvtss2si,                        // Instruction 'vcvtss2si' {AVX|AVX512_F}.
    Vcvtss2usi,                       // Instruction 'vcvtss2usi' {AVX512_F}.
    Vcvttpd2dq,                       // Instruction 'vcvttpd2dq' {AVX|AVX512_F+VL}.
    Vcvttpd2qq,                       // Instruction 'vcvttpd2qq' {AVX512_F+VL}.
    Vcvttpd2udq,                      // Instruction 'vcvttpd2udq' {AVX512_F+VL}.
    Vcvttpd2uqq,                      // Instruction 'vcvttpd2uqq' {AVX512_DQ+VL}.
    Vcvttps2dq,                       // Instruction 'vcvttps2dq' {AVX|AVX512_F+VL}.
    Vcvttps2qq,                       // Instruction 'vcvttps2qq' {AVX512_DQ+VL}.
    Vcvttps2udq,                      // Instruction 'vcvttps2udq' {AVX512_F+VL}.
    Vcvttps2uqq,                      // Instruction 'vcvttps2uqq' {AVX512_DQ+VL}.
    Vcvttsd2si,                       // Instruction 'vcvttsd2si' {AVX|AVX512_F}.
    Vcvttsd2usi,                      // Instruction 'vcvttsd2usi' {AVX512_F}.
    Vcvttss2si,                       // Instruction 'vcvttss2si' {AVX|AVX512_F}.
    Vcvttss2usi,                      // Instruction 'vcvttss2usi' {AVX512_F}.
    Vcvtudq2pd,                       // Instruction 'vcvtudq2pd' {AVX512_F+VL}.
    Vcvtudq2ps,                       // Instruction 'vcvtudq2ps' {AVX512_F+VL}.
    Vcvtuqq2pd,                       // Instruction 'vcvtuqq2pd' {AVX512_DQ+VL}.
    Vcvtuqq2ps,                       // Instruction 'vcvtuqq2ps' {AVX512_DQ+VL}.
    Vcvtusi2sd,                       // Instruction 'vcvtusi2sd' {AVX512_F}.
    Vcvtusi2ss,                       // Instruction 'vcvtusi2ss' {AVX512_F}.
    Vdbpsadbw,                        // Instruction 'vdbpsadbw' {AVX512_BW+VL}.
    Vdivpd,                           // Instruction 'vdivpd' {AVX|AVX512_F+VL}.
    Vdivps,                           // Instruction 'vdivps' {AVX|AVX512_F+VL}.
    Vdivsd,                           // Instruction 'vdivsd' {AVX|AVX512_F}.
    Vdivss,                           // Instruction 'vdivss' {AVX|AVX512_F}.
    Vdpbf16ps,                        // Instruction 'vdpbf16ps' {AVX512_BF16+VL}.
    Vdppd,                            // Instruction 'vdppd' {AVX}.
    Vdpps,                            // Instruction 'vdpps' {AVX}.
    Verr,                             // Instruction 'verr'.
    Verw,                             // Instruction 'verw'.
    Vexp2pd,                          // Instruction 'vexp2pd' {AVX512_ERI}.
    Vexp2ps,                          // Instruction 'vexp2ps' {AVX512_ERI}.
    Vexpandpd,                        // Instruction 'vexpandpd' {AVX512_F+VL}.
    Vexpandps,                        // Instruction 'vexpandps' {AVX512_F+VL}.
    Vextractf128,                     // Instruction 'vextractf128' {AVX}.
    Vextractf32x4,                    // Instruction 'vextractf32x4' {AVX512_F+VL}.
    Vextractf32x8,                    // Instruction 'vextractf32x8' {AVX512_DQ}.
    Vextractf64x2,                    // Instruction 'vextractf64x2' {AVX512_DQ+VL}.
    Vextractf64x4,                    // Instruction 'vextractf64x4' {AVX512_F}.
    Vextracti128,                     // Instruction 'vextracti128' {AVX2}.
    Vextracti32x4,                    // Instruction 'vextracti32x4' {AVX512_F+VL}.
    Vextracti32x8,                    // Instruction 'vextracti32x8' {AVX512_DQ}.
    Vextracti64x2,                    // Instruction 'vextracti64x2' {AVX512_DQ+VL}.
    Vextracti64x4,                    // Instruction 'vextracti64x4' {AVX512_F}.
    Vextractps,                       // Instruction 'vextractps' {AVX|AVX512_F}.
    Vfixupimmpd,                      // Instruction 'vfixupimmpd' {AVX512_F+VL}.
    Vfixupimmps,                      // Instruction 'vfixupimmps' {AVX512_F+VL}.
    Vfixupimmsd,                      // Instruction 'vfixupimmsd' {AVX512_F}.
    Vfixupimmss,                      // Instruction 'vfixupimmss' {AVX512_F}.
    Vfmadd132pd,                      // Instruction 'vfmadd132pd' {FMA|AVX512_F+VL}.
    Vfmadd132ps,                      // Instruction 'vfmadd132ps' {FMA|AVX512_F+VL}.
    Vfmadd132sd,                      // Instruction 'vfmadd132sd' {FMA|AVX512_F}.
    Vfmadd132ss,                      // Instruction 'vfmadd132ss' {FMA|AVX512_F}.
    Vfmadd213pd,                      // Instruction 'vfmadd213pd' {FMA|AVX512_F+VL}.
    Vfmadd213ps,                      // Instruction 'vfmadd213ps' {FMA|AVX512_F+VL}.
    Vfmadd213sd,                      // Instruction 'vfmadd213sd' {FMA|AVX512_F}.
    Vfmadd213ss,                      // Instruction 'vfmadd213ss' {FMA|AVX512_F}.
    Vfmadd231pd,                      // Instruction 'vfmadd231pd' {FMA|AVX512_F+VL}.
    Vfmadd231ps,                      // Instruction 'vfmadd231ps' {FMA|AVX512_F+VL}.
    Vfmadd231sd,                      // Instruction 'vfmadd231sd' {FMA|AVX512_F}.
    Vfmadd231ss,                      // Instruction 'vfmadd231ss' {FMA|AVX512_F}.
    Vfmaddpd,                         // Instruction 'vfmaddpd' {FMA4}.
    Vfmaddps,                         // Instruction 'vfmaddps' {FMA4}.
    Vfmaddsd,                         // Instruction 'vfmaddsd' {FMA4}.
    Vfmaddss,                         // Instruction 'vfmaddss' {FMA4}.
    Vfmaddsub132pd,                   // Instruction 'vfmaddsub132pd' {FMA|AVX512_F+VL}.
    Vfmaddsub132ps,                   // Instruction 'vfmaddsub132ps' {FMA|AVX512_F+VL}.
    Vfmaddsub213pd,                   // Instruction 'vfmaddsub213pd' {FMA|AVX512_F+VL}.
    Vfmaddsub213ps,                   // Instruction 'vfmaddsub213ps' {FMA|AVX512_F+VL}.
    Vfmaddsub231pd,                   // Instruction 'vfmaddsub231pd' {FMA|AVX512_F+VL}.
    Vfmaddsub231ps,                   // Instruction 'vfmaddsub231ps' {FMA|AVX512_F+VL}.
    Vfmaddsubpd,                      // Instruction 'vfmaddsubpd' {FMA4}.
    Vfmaddsubps,                      // Instruction 'vfmaddsubps' {FMA4}.
    Vfmsub132pd,                      // Instruction 'vfmsub132pd' {FMA|AVX512_F+VL}.
    Vfmsub132ps,                      // Instruction 'vfmsub132ps' {FMA|AVX512_F+VL}.
    Vfmsub132sd,                      // Instruction 'vfmsub132sd' {FMA|AVX512_F}.
    Vfmsub132ss,                      // Instruction 'vfmsub132ss' {FMA|AVX512_F}.
    Vfmsub213pd,                      // Instruction 'vfmsub213pd' {FMA|AVX512_F+VL}.
    Vfmsub213ps,                      // Instruction 'vfmsub213ps' {FMA|AVX512_F+VL}.
    Vfmsub213sd,                      // Instruction 'vfmsub213sd' {FMA|AVX512_F}.
    Vfmsub213ss,                      // Instruction 'vfmsub213ss' {FMA|AVX512_F}.
    Vfmsub231pd,                      // Instruction 'vfmsub231pd' {FMA|AVX512_F+VL}.
    Vfmsub231ps,                      // Instruction 'vfmsub231ps' {FMA|AVX512_F+VL}.
    Vfmsub231sd,                      // Instruction 'vfmsub231sd' {FMA|AVX512_F}.
    Vfmsub231ss,                      // Instruction 'vfmsub231ss' {FMA|AVX512_F}.
    Vfmsubadd132pd,                   // Instruction 'vfmsubadd132pd' {FMA|AVX512_F+VL}.
    Vfmsubadd132ps,                   // Instruction 'vfmsubadd132ps' {FMA|AVX512_F+VL}.
    Vfmsubadd213pd,                   // Instruction 'vfmsubadd213pd' {FMA|AVX512_F+VL}.
    Vfmsubadd213ps,                   // Instruction 'vfmsubadd213ps' {FMA|AVX512_F+VL}.
    Vfmsubadd231pd,                   // Instruction 'vfmsubadd231pd' {FMA|AVX512_F+VL}.
    Vfmsubadd231ps,                   // Instruction 'vfmsubadd231ps' {FMA|AVX512_F+VL}.
    Vfmsubaddpd,                      // Instruction 'vfmsubaddpd' {FMA4}.
    Vfmsubaddps,                      // Instruction 'vfmsubaddps' {FMA4}.
    Vfmsubpd,                         // Instruction 'vfmsubpd' {FMA4}.
    Vfmsubps,                         // Instruction 'vfmsubps' {FMA4}.
    Vfmsubsd,                         // Instruction 'vfmsubsd' {FMA4}.
    Vfmsubss,                         // Instruction 'vfmsubss' {FMA4}.
    Vfnmadd132pd,                     // Instruction 'vfnmadd132pd' {FMA|AVX512_F+VL}.
    Vfnmadd132ps,                     // Instruction 'vfnmadd132ps' {FMA|AVX512_F+VL}.
    Vfnmadd132sd,                     // Instruction 'vfnmadd132sd' {FMA|AVX512_F}.
    Vfnmadd132ss,                     // Instruction 'vfnmadd132ss' {FMA|AVX512_F}.
    Vfnmadd213pd,                     // Instruction 'vfnmadd213pd' {FMA|AVX512_F+VL}.
    Vfnmadd213ps,                     // Instruction 'vfnmadd213ps' {FMA|AVX512_F+VL}.
    Vfnmadd213sd,                     // Instruction 'vfnmadd213sd' {FMA|AVX512_F}.
    Vfnmadd213ss,                     // Instruction 'vfnmadd213ss' {FMA|AVX512_F}.
    Vfnmadd231pd,                     // Instruction 'vfnmadd231pd' {FMA|AVX512_F+VL}.
    Vfnmadd231ps,                     // Instruction 'vfnmadd231ps' {FMA|AVX512_F+VL}.
    Vfnmadd231sd,                     // Instruction 'vfnmadd231sd' {FMA|AVX512_F}.
    Vfnmadd231ss,                     // Instruction 'vfnmadd231ss' {FMA|AVX512_F}.
    Vfnmaddpd,                        // Instruction 'vfnmaddpd' {FMA4}.
    Vfnmaddps,                        // Instruction 'vfnmaddps' {FMA4}.
    Vfnmaddsd,                        // Instruction 'vfnmaddsd' {FMA4}.
    Vfnmaddss,                        // Instruction 'vfnmaddss' {FMA4}.
    Vfnmsub132pd,                     // Instruction 'vfnmsub132pd' {FMA|AVX512_F+VL}.
    Vfnmsub132ps,                     // Instruction 'vfnmsub132ps' {FMA|AVX512_F+VL}.
    Vfnmsub132sd,                     // Instruction 'vfnmsub132sd' {FMA|AVX512_F}.
    Vfnmsub132ss,                     // Instruction 'vfnmsub132ss' {FMA|AVX512_F}.
    Vfnmsub213pd,                     // Instruction 'vfnmsub213pd' {FMA|AVX512_F+VL}.
    Vfnmsub213ps,                     // Instruction 'vfnmsub213ps' {FMA|AVX512_F+VL}.
    Vfnmsub213sd,                     // Instruction 'vfnmsub213sd' {FMA|AVX512_F}.
    Vfnmsub213ss,                     // Instruction 'vfnmsub213ss' {FMA|AVX512_F}.
    Vfnmsub231pd,                     // Instruction 'vfnmsub231pd' {FMA|AVX512_F+VL}.
    Vfnmsub231ps,                     // Instruction 'vfnmsub231ps' {FMA|AVX512_F+VL}.
    Vfnmsub231sd,                     // Instruction 'vfnmsub231sd' {FMA|AVX512_F}.
    Vfnmsub231ss,                     // Instruction 'vfnmsub231ss' {FMA|AVX512_F}.
    Vfnmsubpd,                        // Instruction 'vfnmsubpd' {FMA4}.
    Vfnmsubps,                        // Instruction 'vfnmsubps' {FMA4}.
    Vfnmsubsd,                        // Instruction 'vfnmsubsd' {FMA4}.
    Vfnmsubss,                        // Instruction 'vfnmsubss' {FMA4}.
    Vfpclasspd,                       // Instruction 'vfpclasspd' {AVX512_DQ+VL}.
    Vfpclassps,                       // Instruction 'vfpclassps' {AVX512_DQ+VL}.
    Vfpclasssd,                       // Instruction 'vfpclasssd' {AVX512_DQ}.
    Vfpclassss,                       // Instruction 'vfpclassss' {AVX512_DQ}.
    Vfrczpd,                          // Instruction 'vfrczpd' {XOP}.
    Vfrczps,                          // Instruction 'vfrczps' {XOP}.
    Vfrczsd,                          // Instruction 'vfrczsd' {XOP}.
    Vfrczss,                          // Instruction 'vfrczss' {XOP}.
    Vgatherdpd,                       // Instruction 'vgatherdpd' {AVX2|AVX512_F+VL}.
    Vgatherdps,                       // Instruction 'vgatherdps' {AVX2|AVX512_F+VL}.
    Vgatherpf0dpd,                    // Instruction 'vgatherpf0dpd' {AVX512_PFI}.
    Vgatherpf0dps,                    // Instruction 'vgatherpf0dps' {AVX512_PFI}.
    Vgatherpf0qpd,                    // Instruction 'vgatherpf0qpd' {AVX512_PFI}.
    Vgatherpf0qps,                    // Instruction 'vgatherpf0qps' {AVX512_PFI}.
    Vgatherpf1dpd,                    // Instruction 'vgatherpf1dpd' {AVX512_PFI}.
    Vgatherpf1dps,                    // Instruction 'vgatherpf1dps' {AVX512_PFI}.
    Vgatherpf1qpd,                    // Instruction 'vgatherpf1qpd' {AVX512_PFI}.
    Vgatherpf1qps,                    // Instruction 'vgatherpf1qps' {AVX512_PFI}.
    Vgatherqpd,                       // Instruction 'vgatherqpd' {AVX2|AVX512_F+VL}.
    Vgatherqps,                       // Instruction 'vgatherqps' {AVX2|AVX512_F+VL}.
    Vgetexppd,                        // Instruction 'vgetexppd' {AVX512_F+VL}.
    Vgetexpps,                        // Instruction 'vgetexpps' {AVX512_F+VL}.
    Vgetexpsd,                        // Instruction 'vgetexpsd' {AVX512_F}.
    Vgetexpss,                        // Instruction 'vgetexpss' {AVX512_F}.
    Vgetmantpd,                       // Instruction 'vgetmantpd' {AVX512_F+VL}.
    Vgetmantps,                       // Instruction 'vgetmantps' {AVX512_F+VL}.
    Vgetmantsd,                       // Instruction 'vgetmantsd' {AVX512_F}.
    Vgetmantss,                       // Instruction 'vgetmantss' {AVX512_F}.
    Vgf2p8affineinvqb,                // Instruction 'vgf2p8affineinvqb' {AVX|AVX512_F+VL & GFNI}.
    Vgf2p8affineqb,                   // Instruction 'vgf2p8affineqb' {AVX|AVX512_F+VL & GFNI}.
    Vgf2p8mulb,                       // Instruction 'vgf2p8mulb' {AVX|AVX512_F+VL & GFNI}.
    Vhaddpd,                          // Instruction 'vhaddpd' {AVX}.
    Vhaddps,                          // Instruction 'vhaddps' {AVX}.
    Vhsubpd,                          // Instruction 'vhsubpd' {AVX}.
    Vhsubps,                          // Instruction 'vhsubps' {AVX}.
    Vinsertf128,                      // Instruction 'vinsertf128' {AVX}.
    Vinsertf32x4,                     // Instruction 'vinsertf32x4' {AVX512_F+VL}.
    Vinsertf32x8,                     // Instruction 'vinsertf32x8' {AVX512_DQ}.
    Vinsertf64x2,                     // Instruction 'vinsertf64x2' {AVX512_DQ+VL}.
    Vinsertf64x4,                     // Instruction 'vinsertf64x4' {AVX512_F}.
    Vinserti128,                      // Instruction 'vinserti128' {AVX2}.
    Vinserti32x4,                     // Instruction 'vinserti32x4' {AVX512_F+VL}.
    Vinserti32x8,                     // Instruction 'vinserti32x8' {AVX512_DQ}.
    Vinserti64x2,                     // Instruction 'vinserti64x2' {AVX512_DQ+VL}.
    Vinserti64x4,                     // Instruction 'vinserti64x4' {AVX512_F}.
    Vinsertps,                        // Instruction 'vinsertps' {AVX|AVX512_F}.
    Vlddqu,                           // Instruction 'vlddqu' {AVX}.
    Vldmxcsr,                         // Instruction 'vldmxcsr' {AVX}.
    Vmaskmovdqu,                      // Instruction 'vmaskmovdqu' {AVX}.
    Vmaskmovpd,                       // Instruction 'vmaskmovpd' {AVX}.
    Vmaskmovps,                       // Instruction 'vmaskmovps' {AVX}.
    Vmaxpd,                           // Instruction 'vmaxpd' {AVX|AVX512_F+VL}.
    Vmaxps,                           // Instruction 'vmaxps' {AVX|AVX512_F+VL}.
    Vmaxsd,                           // Instruction 'vmaxsd' {AVX|AVX512_F+VL}.
    Vmaxss,                           // Instruction 'vmaxss' {AVX|AVX512_F+VL}.
    Vmcall,                           // Instruction 'vmcall' {VMX}.
    Vmclear,                          // Instruction 'vmclear' {VMX}.
    Vmfunc,                           // Instruction 'vmfunc' {VMX}.
    Vminpd,                           // Instruction 'vminpd' {AVX|AVX512_F+VL}.
    Vminps,                           // Instruction 'vminps' {AVX|AVX512_F+VL}.
    Vminsd,                           // Instruction 'vminsd' {AVX|AVX512_F+VL}.
    Vminss,                           // Instruction 'vminss' {AVX|AVX512_F+VL}.
    Vmlaunch,                         // Instruction 'vmlaunch' {VMX}.
    Vmload,                           // Instruction 'vmload' {SVM}.
    Vmmcall,                          // Instruction 'vmmcall' {SVM}.
    Vmovapd,                          // Instruction 'vmovapd' {AVX|AVX512_F+VL}.
    Vmovaps,                          // Instruction 'vmovaps' {AVX|AVX512_F+VL}.
    Vmovd,                            // Instruction 'vmovd' {AVX|AVX512_F}.
    Vmovddup,                         // Instruction 'vmovddup' {AVX|AVX512_F+VL}.
    Vmovdqa,                          // Instruction 'vmovdqa' {AVX}.
    Vmovdqa32,                        // Instruction 'vmovdqa32' {AVX512_F+VL}.
    Vmovdqa64,                        // Instruction 'vmovdqa64' {AVX512_F+VL}.
    Vmovdqu,                          // Instruction 'vmovdqu' {AVX}.
    Vmovdqu16,                        // Instruction 'vmovdqu16' {AVX512_BW+VL}.
    Vmovdqu32,                        // Instruction 'vmovdqu32' {AVX512_F+VL}.
    Vmovdqu64,                        // Instruction 'vmovdqu64' {AVX512_F+VL}.
    Vmovdqu8,                         // Instruction 'vmovdqu8' {AVX512_BW+VL}.
    Vmovhlps,                         // Instruction 'vmovhlps' {AVX|AVX512_F}.
    Vmovhpd,                          // Instruction 'vmovhpd' {AVX|AVX512_F}.
    Vmovhps,                          // Instruction 'vmovhps' {AVX|AVX512_F}.
    Vmovlhps,                         // Instruction 'vmovlhps' {AVX|AVX512_F}.
    Vmovlpd,                          // Instruction 'vmovlpd' {AVX|AVX512_F}.
    Vmovlps,                          // Instruction 'vmovlps' {AVX|AVX512_F}.
    Vmovmskpd,                        // Instruction 'vmovmskpd' {AVX}.
    Vmovmskps,                        // Instruction 'vmovmskps' {AVX}.
    Vmovntdq,                         // Instruction 'vmovntdq' {AVX|AVX512_F+VL}.
    Vmovntdqa,                        // Instruction 'vmovntdqa' {AVX|AVX2|AVX512_F+VL}.
    Vmovntpd,                         // Instruction 'vmovntpd' {AVX|AVX512_F+VL}.
    Vmovntps,                         // Instruction 'vmovntps' {AVX|AVX512_F+VL}.
    Vmovq,                            // Instruction 'vmovq' {AVX|AVX512_F}.
    Vmovsd,                           // Instruction 'vmovsd' {AVX|AVX512_F}.
    Vmovshdup,                        // Instruction 'vmovshdup' {AVX|AVX512_F+VL}.
    Vmovsldup,                        // Instruction 'vmovsldup' {AVX|AVX512_F+VL}.
    Vmovss,                           // Instruction 'vmovss' {AVX|AVX512_F}.
    Vmovupd,                          // Instruction 'vmovupd' {AVX|AVX512_F+VL}.
    Vmovups,                          // Instruction 'vmovups' {AVX|AVX512_F+VL}.
    Vmpsadbw,                         // Instruction 'vmpsadbw' {AVX|AVX2}.
    Vmptrld,                          // Instruction 'vmptrld' {VMX}.
    Vmptrst,                          // Instruction 'vmptrst' {VMX}.
    Vmread,                           // Instruction 'vmread' {VMX}.
    Vmresume,                         // Instruction 'vmresume' {VMX}.
    Vmrun,                            // Instruction 'vmrun' {SVM}.
    Vmsave,                           // Instruction 'vmsave' {SVM}.
    Vmulpd,                           // Instruction 'vmulpd' {AVX|AVX512_F+VL}.
    Vmulps,                           // Instruction 'vmulps' {AVX|AVX512_F+VL}.
    Vmulsd,                           // Instruction 'vmulsd' {AVX|AVX512_F}.
    Vmulss,                           // Instruction 'vmulss' {AVX|AVX512_F}.
    Vmwrite,                          // Instruction 'vmwrite' {VMX}.
    Vmxon,                            // Instruction 'vmxon' {VMX}.
    Vorpd,                            // Instruction 'vorpd' {AVX|AVX512_DQ+VL}.
    Vorps,                            // Instruction 'vorps' {AVX|AVX512_DQ+VL}.
    Vp4dpwssd,                        // Instruction 'vp4dpwssd' {AVX512_4VNNIW}.
    Vp4dpwssds,                       // Instruction 'vp4dpwssds' {AVX512_4VNNIW}.
    Vpabsb,                           // Instruction 'vpabsb' {AVX|AVX2|AVX512_BW+VL}.
    Vpabsd,                           // Instruction 'vpabsd' {AVX|AVX2|AVX512_F+VL}.
    Vpabsq,                           // Instruction 'vpabsq' {AVX512_F+VL}.
    Vpabsw,                           // Instruction 'vpabsw' {AVX|AVX2|AVX512_BW+VL}.
    Vpackssdw,                        // Instruction 'vpackssdw' {AVX|AVX2|AVX512_BW+VL}.
    Vpacksswb,                        // Instruction 'vpacksswb' {AVX|AVX2|AVX512_BW+VL}.
    Vpackusdw,                        // Instruction 'vpackusdw' {AVX|AVX2|AVX512_BW+VL}.
    Vpackuswb,                        // Instruction 'vpackuswb' {AVX|AVX2|AVX512_BW+VL}.
    Vpaddb,                           // Instruction 'vpaddb' {AVX|AVX2|AVX512_BW+VL}.
    Vpaddd,                           // Instruction 'vpaddd' {AVX|AVX2|AVX512_F+VL}.
    Vpaddq,                           // Instruction 'vpaddq' {AVX|AVX2|AVX512_F+VL}.
    Vpaddsb,                          // Instruction 'vpaddsb' {AVX|AVX2|AVX512_BW+VL}.
    Vpaddsw,                          // Instruction 'vpaddsw' {AVX|AVX2|AVX512_BW+VL}.
    Vpaddusb,                         // Instruction 'vpaddusb' {AVX|AVX2|AVX512_BW+VL}.
    Vpaddusw,                         // Instruction 'vpaddusw' {AVX|AVX2|AVX512_BW+VL}.
    Vpaddw,                           // Instruction 'vpaddw' {AVX|AVX2|AVX512_BW+VL}.
    Vpalignr,                         // Instruction 'vpalignr' {AVX|AVX2|AVX512_BW+VL}.
    Vpand,                            // Instruction 'vpand' {AVX|AVX2}.
    Vpandd,                           // Instruction 'vpandd' {AVX512_F+VL}.
    Vpandn,                           // Instruction 'vpandn' {AVX|AVX2}.
    Vpandnd,                          // Instruction 'vpandnd' {AVX512_F+VL}.
    Vpandnq,                          // Instruction 'vpandnq' {AVX512_F+VL}.
    Vpandq,                           // Instruction 'vpandq' {AVX512_F+VL}.
    Vpavgb,                           // Instruction 'vpavgb' {AVX|AVX2|AVX512_BW+VL}.
    Vpavgw,                           // Instruction 'vpavgw' {AVX|AVX2|AVX512_BW+VL}.
    Vpblendd,                         // Instruction 'vpblendd' {AVX2}.
    Vpblendvb,                        // Instruction 'vpblendvb' {AVX|AVX2}.
    Vpblendw,                         // Instruction 'vpblendw' {AVX|AVX2}.
    Vpbroadcastb,                     // Instruction 'vpbroadcastb' {AVX2|AVX512_BW+VL}.
    Vpbroadcastd,                     // Instruction 'vpbroadcastd' {AVX2|AVX512_F+VL}.
    Vpbroadcastmb2d,                  // Instruction 'vpbroadcastmb2d' {AVX512_CDI+VL}.
    Vpbroadcastmb2q,                  // Instruction 'vpbroadcastmb2q' {AVX512_CDI+VL}.
    Vpbroadcastq,                     // Instruction 'vpbroadcastq' {AVX2|AVX512_F+VL}.
    Vpbroadcastw,                     // Instruction 'vpbroadcastw' {AVX2|AVX512_BW+VL}.
    Vpclmulqdq,                       // Instruction 'vpclmulqdq' {AVX|AVX512_F+VL & PCLMULQDQ|VPCLMULQDQ}.
    Vpcmov,                           // Instruction 'vpcmov' {XOP}.
    Vpcmpb,                           // Instruction 'vpcmpb' {AVX512_BW+VL}.
    Vpcmpd,                           // Instruction 'vpcmpd' {AVX512_F+VL}.
    Vpcmpeqb,                         // Instruction 'vpcmpeqb' {AVX|AVX2|AVX512_BW+VL}.
    Vpcmpeqd,                         // Instruction 'vpcmpeqd' {AVX|AVX2|AVX512_F+VL}.
    Vpcmpeqq,                         // Instruction 'vpcmpeqq' {AVX|AVX2|AVX512_F+VL}.
    Vpcmpeqw,                         // Instruction 'vpcmpeqw' {AVX|AVX2|AVX512_BW+VL}.
    Vpcmpestri,                       // Instruction 'vpcmpestri' {AVX}.
    Vpcmpestrm,                       // Instruction 'vpcmpestrm' {AVX}.
    Vpcmpgtb,                         // Instruction 'vpcmpgtb' {AVX|AVX2|AVX512_BW+VL}.
    Vpcmpgtd,                         // Instruction 'vpcmpgtd' {AVX|AVX2|AVX512_F+VL}.
    Vpcmpgtq,                         // Instruction 'vpcmpgtq' {AVX|AVX2|AVX512_F+VL}.
    Vpcmpgtw,                         // Instruction 'vpcmpgtw' {AVX|AVX2|AVX512_BW+VL}.
    Vpcmpistri,                       // Instruction 'vpcmpistri' {AVX}.
    Vpcmpistrm,                       // Instruction 'vpcmpistrm' {AVX}.
    Vpcmpq,                           // Instruction 'vpcmpq' {AVX512_F+VL}.
    Vpcmpub,                          // Instruction 'vpcmpub' {AVX512_BW+VL}.
    Vpcmpud,                          // Instruction 'vpcmpud' {AVX512_F+VL}.
    Vpcmpuq,                          // Instruction 'vpcmpuq' {AVX512_F+VL}.
    Vpcmpuw,                          // Instruction 'vpcmpuw' {AVX512_BW+VL}.
    Vpcmpw,                           // Instruction 'vpcmpw' {AVX512_BW+VL}.
    Vpcomb,                           // Instruction 'vpcomb' {XOP}.
    Vpcomd,                           // Instruction 'vpcomd' {XOP}.
    Vpcompressb,                      // Instruction 'vpcompressb' {AVX512_VBMI2+VL}.
    Vpcompressd,                      // Instruction 'vpcompressd' {AVX512_F+VL}.
    Vpcompressq,                      // Instruction 'vpcompressq' {AVX512_F+VL}.
    Vpcompressw,                      // Instruction 'vpcompressw' {AVX512_VBMI2+VL}.
    Vpcomq,                           // Instruction 'vpcomq' {XOP}.
    Vpcomub,                          // Instruction 'vpcomub' {XOP}.
    Vpcomud,                          // Instruction 'vpcomud' {XOP}.
    Vpcomuq,                          // Instruction 'vpcomuq' {XOP}.
    Vpcomuw,                          // Instruction 'vpcomuw' {XOP}.
    Vpcomw,                           // Instruction 'vpcomw' {XOP}.
    Vpconflictd,                      // Instruction 'vpconflictd' {AVX512_CDI+VL}.
    Vpconflictq,                      // Instruction 'vpconflictq' {AVX512_CDI+VL}.
    Vpdpbusd,                         // Instruction 'vpdpbusd' {AVX512_VNNI+VL}.
    Vpdpbusds,                        // Instruction 'vpdpbusds' {AVX512_VNNI+VL}.
    Vpdpwssd,                         // Instruction 'vpdpwssd' {AVX512_VNNI+VL}.
    Vpdpwssds,                        // Instruction 'vpdpwssds' {AVX512_VNNI+VL}.
    Vperm2f128,                       // Instruction 'vperm2f128' {AVX}.
    Vperm2i128,                       // Instruction 'vperm2i128' {AVX2}.
    Vpermb,                           // Instruction 'vpermb' {AVX512_VBMI+VL}.
    Vpermd,                           // Instruction 'vpermd' {AVX2|AVX512_F+VL}.
    Vpermi2b,                         // Instruction 'vpermi2b' {AVX512_VBMI+VL}.
    Vpermi2d,                         // Instruction 'vpermi2d' {AVX512_F+VL}.
    Vpermi2pd,                        // Instruction 'vpermi2pd' {AVX512_F+VL}.
    Vpermi2ps,                        // Instruction 'vpermi2ps' {AVX512_F+VL}.
    Vpermi2q,                         // Instruction 'vpermi2q' {AVX512_F+VL}.
    Vpermi2w,                         // Instruction 'vpermi2w' {AVX512_BW+VL}.
    Vpermil2pd,                       // Instruction 'vpermil2pd' {XOP}.
    Vpermil2ps,                       // Instruction 'vpermil2ps' {XOP}.
    Vpermilpd,                        // Instruction 'vpermilpd' {AVX|AVX512_F+VL}.
    Vpermilps,                        // Instruction 'vpermilps' {AVX|AVX512_F+VL}.
    Vpermpd,                          // Instruction 'vpermpd' {AVX2|AVX512_F+VL}.
    Vpermps,                          // Instruction 'vpermps' {AVX2|AVX512_F+VL}.
    Vpermq,                           // Instruction 'vpermq' {AVX2|AVX512_F+VL}.
    Vpermt2b,                         // Instruction 'vpermt2b' {AVX512_VBMI+VL}.
    Vpermt2d,                         // Instruction 'vpermt2d' {AVX512_F+VL}.
    Vpermt2pd,                        // Instruction 'vpermt2pd' {AVX512_F+VL}.
    Vpermt2ps,                        // Instruction 'vpermt2ps' {AVX512_F+VL}.
    Vpermt2q,                         // Instruction 'vpermt2q' {AVX512_F+VL}.
    Vpermt2w,                         // Instruction 'vpermt2w' {AVX512_BW+VL}.
    Vpermw,                           // Instruction 'vpermw' {AVX512_BW+VL}.
    Vpexpandb,                        // Instruction 'vpexpandb' {AVX512_VBMI2+VL}.
    Vpexpandd,                        // Instruction 'vpexpandd' {AVX512_F+VL}.
    Vpexpandq,                        // Instruction 'vpexpandq' {AVX512_F+VL}.
    Vpexpandw,                        // Instruction 'vpexpandw' {AVX512_VBMI2+VL}.
    Vpextrb,                          // Instruction 'vpextrb' {AVX|AVX512_BW}.
    Vpextrd,                          // Instruction 'vpextrd' {AVX|AVX512_DQ}.
    Vpextrq,                          // Instruction 'vpextrq' {AVX|AVX512_DQ} (X64).
    Vpextrw,                          // Instruction 'vpextrw' {AVX|AVX512_BW}.
    Vpgatherdd,                       // Instruction 'vpgatherdd' {AVX2|AVX512_F+VL}.
    Vpgatherdq,                       // Instruction 'vpgatherdq' {AVX2|AVX512_F+VL}.
    Vpgatherqd,                       // Instruction 'vpgatherqd' {AVX2|AVX512_F+VL}.
    Vpgatherqq,                       // Instruction 'vpgatherqq' {AVX2|AVX512_F+VL}.
    Vphaddbd,                         // Instruction 'vphaddbd' {XOP}.
    Vphaddbq,                         // Instruction 'vphaddbq' {XOP}.
    Vphaddbw,                         // Instruction 'vphaddbw' {XOP}.
    Vphaddd,                          // Instruction 'vphaddd' {AVX|AVX2}.
    Vphadddq,                         // Instruction 'vphadddq' {XOP}.
    Vphaddsw,                         // Instruction 'vphaddsw' {AVX|AVX2}.
    Vphaddubd,                        // Instruction 'vphaddubd' {XOP}.
    Vphaddubq,                        // Instruction 'vphaddubq' {XOP}.
    Vphaddubw,                        // Instruction 'vphaddubw' {XOP}.
    Vphaddudq,                        // Instruction 'vphaddudq' {XOP}.
    Vphadduwd,                        // Instruction 'vphadduwd' {XOP}.
    Vphadduwq,                        // Instruction 'vphadduwq' {XOP}.
    Vphaddw,                          // Instruction 'vphaddw' {AVX|AVX2}.
    Vphaddwd,                         // Instruction 'vphaddwd' {XOP}.
    Vphaddwq,                         // Instruction 'vphaddwq' {XOP}.
    Vphminposuw,                      // Instruction 'vphminposuw' {AVX}.
    Vphsubbw,                         // Instruction 'vphsubbw' {XOP}.
    Vphsubd,                          // Instruction 'vphsubd' {AVX|AVX2}.
    Vphsubdq,                         // Instruction 'vphsubdq' {XOP}.
    Vphsubsw,                         // Instruction 'vphsubsw' {AVX|AVX2}.
    Vphsubw,                          // Instruction 'vphsubw' {AVX|AVX2}.
    Vphsubwd,                         // Instruction 'vphsubwd' {XOP}.
    Vpinsrb,                          // Instruction 'vpinsrb' {AVX|AVX512_BW}.
    Vpinsrd,                          // Instruction 'vpinsrd' {AVX|AVX512_DQ}.
    Vpinsrq,                          // Instruction 'vpinsrq' {AVX|AVX512_DQ} (X64).
    Vpinsrw,                          // Instruction 'vpinsrw' {AVX|AVX512_BW}.
    Vplzcntd,                         // Instruction 'vplzcntd' {AVX512_CDI+VL}.
    Vplzcntq,                         // Instruction 'vplzcntq' {AVX512_CDI+VL}.
    Vpmacsdd,                         // Instruction 'vpmacsdd' {XOP}.
    Vpmacsdqh,                        // Instruction 'vpmacsdqh' {XOP}.
    Vpmacsdql,                        // Instruction 'vpmacsdql' {XOP}.
    Vpmacssdd,                        // Instruction 'vpmacssdd' {XOP}.
    Vpmacssdqh,                       // Instruction 'vpmacssdqh' {XOP}.
    Vpmacssdql,                       // Instruction 'vpmacssdql' {XOP}.
    Vpmacsswd,                        // Instruction 'vpmacsswd' {XOP}.
    Vpmacssww,                        // Instruction 'vpmacssww' {XOP}.
    Vpmacswd,                         // Instruction 'vpmacswd' {XOP}.
    Vpmacsww,                         // Instruction 'vpmacsww' {XOP}.
    Vpmadcsswd,                       // Instruction 'vpmadcsswd' {XOP}.
    Vpmadcswd,                        // Instruction 'vpmadcswd' {XOP}.
    Vpmadd52huq,                      // Instruction 'vpmadd52huq' {AVX512_IFMA+VL}.
    Vpmadd52luq,                      // Instruction 'vpmadd52luq' {AVX512_IFMA+VL}.
    Vpmaddubsw,                       // Instruction 'vpmaddubsw' {AVX|AVX2|AVX512_BW+VL}.
    Vpmaddwd,                         // Instruction 'vpmaddwd' {AVX|AVX2|AVX512_BW+VL}.
    Vpmaskmovd,                       // Instruction 'vpmaskmovd' {AVX2}.
    Vpmaskmovq,                       // Instruction 'vpmaskmovq' {AVX2}.
    Vpmaxsb,                          // Instruction 'vpmaxsb' {AVX|AVX2|AVX512_BW+VL}.
    Vpmaxsd,                          // Instruction 'vpmaxsd' {AVX|AVX2|AVX512_F+VL}.
    Vpmaxsq,                          // Instruction 'vpmaxsq' {AVX512_F+VL}.
    Vpmaxsw,                          // Instruction 'vpmaxsw' {AVX|AVX2|AVX512_BW+VL}.
    Vpmaxub,                          // Instruction 'vpmaxub' {AVX|AVX2|AVX512_BW+VL}.
    Vpmaxud,                          // Instruction 'vpmaxud' {AVX|AVX2|AVX512_F+VL}.
    Vpmaxuq,                          // Instruction 'vpmaxuq' {AVX512_F+VL}.
    Vpmaxuw,                          // Instruction 'vpmaxuw' {AVX|AVX2|AVX512_BW+VL}.
    Vpminsb,                          // Instruction 'vpminsb' {AVX|AVX2|AVX512_BW+VL}.
    Vpminsd,                          // Instruction 'vpminsd' {AVX|AVX2|AVX512_F+VL}.
    Vpminsq,                          // Instruction 'vpminsq' {AVX512_F+VL}.
    Vpminsw,                          // Instruction 'vpminsw' {AVX|AVX2|AVX512_BW+VL}.
    Vpminub,                          // Instruction 'vpminub' {AVX|AVX2|AVX512_BW+VL}.
    Vpminud,                          // Instruction 'vpminud' {AVX|AVX2|AVX512_F+VL}.
    Vpminuq,                          // Instruction 'vpminuq' {AVX512_F+VL}.
    Vpminuw,                          // Instruction 'vpminuw' {AVX|AVX2|AVX512_BW+VL}.
    Vpmovb2m,                         // Instruction 'vpmovb2m' {AVX512_BW+VL}.
    Vpmovd2m,                         // Instruction 'vpmovd2m' {AVX512_DQ+VL}.
    Vpmovdb,                          // Instruction 'vpmovdb' {AVX512_F+VL}.
    Vpmovdw,                          // Instruction 'vpmovdw' {AVX512_F+VL}.
    Vpmovm2b,                         // Instruction 'vpmovm2b' {AVX512_BW+VL}.
    Vpmovm2d,                         // Instruction 'vpmovm2d' {AVX512_DQ+VL}.
    Vpmovm2q,                         // Instruction 'vpmovm2q' {AVX512_DQ+VL}.
    Vpmovm2w,                         // Instruction 'vpmovm2w' {AVX512_BW+VL}.
    Vpmovmskb,                        // Instruction 'vpmovmskb' {AVX|AVX2}.
    Vpmovq2m,                         // Instruction 'vpmovq2m' {AVX512_DQ+VL}.
    Vpmovqb,                          // Instruction 'vpmovqb' {AVX512_F+VL}.
    Vpmovqd,                          // Instruction 'vpmovqd' {AVX512_F+VL}.
    Vpmovqw,                          // Instruction 'vpmovqw' {AVX512_F+VL}.
    Vpmovsdb,                         // Instruction 'vpmovsdb' {AVX512_F+VL}.
    Vpmovsdw,                         // Instruction 'vpmovsdw' {AVX512_F+VL}.
    Vpmovsqb,                         // Instruction 'vpmovsqb' {AVX512_F+VL}.
    Vpmovsqd,                         // Instruction 'vpmovsqd' {AVX512_F+VL}.
    Vpmovsqw,                         // Instruction 'vpmovsqw' {AVX512_F+VL}.
    Vpmovswb,                         // Instruction 'vpmovswb' {AVX512_BW+VL}.
    Vpmovsxbd,                        // Instruction 'vpmovsxbd' {AVX|AVX2|AVX512_F+VL}.
    Vpmovsxbq,                        // Instruction 'vpmovsxbq' {AVX|AVX2|AVX512_F+VL}.
    Vpmovsxbw,                        // Instruction 'vpmovsxbw' {AVX|AVX2|AVX512_BW+VL}.
    Vpmovsxdq,                        // Instruction 'vpmovsxdq' {AVX|AVX2|AVX512_F+VL}.
    Vpmovsxwd,                        // Instruction 'vpmovsxwd' {AVX|AVX2|AVX512_F+VL}.
    Vpmovsxwq,                        // Instruction 'vpmovsxwq' {AVX|AVX2|AVX512_F+VL}.
    Vpmovusdb,                        // Instruction 'vpmovusdb' {AVX512_F+VL}.
    Vpmovusdw,                        // Instruction 'vpmovusdw' {AVX512_F+VL}.
    Vpmovusqb,                        // Instruction 'vpmovusqb' {AVX512_F+VL}.
    Vpmovusqd,                        // Instruction 'vpmovusqd' {AVX512_F+VL}.
    Vpmovusqw,                        // Instruction 'vpmovusqw' {AVX512_F+VL}.
    Vpmovuswb,                        // Instruction 'vpmovuswb' {AVX512_BW+VL}.
    Vpmovw2m,                         // Instruction 'vpmovw2m' {AVX512_BW+VL}.
    Vpmovwb,                          // Instruction 'vpmovwb' {AVX512_BW+VL}.
    Vpmovzxbd,                        // Instruction 'vpmovzxbd' {AVX|AVX2|AVX512_F+VL}.
    Vpmovzxbq,                        // Instruction 'vpmovzxbq' {AVX|AVX2|AVX512_F+VL}.
    Vpmovzxbw,                        // Instruction 'vpmovzxbw' {AVX|AVX2|AVX512_BW+VL}.
    Vpmovzxdq,                        // Instruction 'vpmovzxdq' {AVX|AVX2|AVX512_F+VL}.
    Vpmovzxwd,                        // Instruction 'vpmovzxwd' {AVX|AVX2|AVX512_F+VL}.
    Vpmovzxwq,                        // Instruction 'vpmovzxwq' {AVX|AVX2|AVX512_F+VL}.
    Vpmuldq,                          // Instruction 'vpmuldq' {AVX|AVX2|AVX512_F+VL}.
    Vpmulhrsw,                        // Instruction 'vpmulhrsw' {AVX|AVX2|AVX512_BW+VL}.
    Vpmulhuw,                         // Instruction 'vpmulhuw' {AVX|AVX2|AVX512_BW+VL}.
    Vpmulhw,                          // Instruction 'vpmulhw' {AVX|AVX2|AVX512_BW+VL}.
    Vpmulld,                          // Instruction 'vpmulld' {AVX|AVX2|AVX512_F+VL}.
    Vpmullq,                          // Instruction 'vpmullq' {AVX512_DQ+VL}.
    Vpmullw,                          // Instruction 'vpmullw' {AVX|AVX2|AVX512_BW+VL}.
    Vpmultishiftqb,                   // Instruction 'vpmultishiftqb' {AVX512_VBMI+VL}.
    Vpmuludq,                         // Instruction 'vpmuludq' {AVX|AVX2|AVX512_F+VL}.
    Vpopcntb,                         // Instruction 'vpopcntb' {AVX512_BITALG+VL}.
    Vpopcntd,                         // Instruction 'vpopcntd' {AVX512_VPOPCNTDQ+VL}.
    Vpopcntq,                         // Instruction 'vpopcntq' {AVX512_VPOPCNTDQ+VL}.
    Vpopcntw,                         // Instruction 'vpopcntw' {AVX512_BITALG+VL}.
    Vpor,                             // Instruction 'vpor' {AVX|AVX2}.
    Vpord,                            // Instruction 'vpord' {AVX512_F+VL}.
    Vporq,                            // Instruction 'vporq' {AVX512_F+VL}.
    Vpperm,                           // Instruction 'vpperm' {XOP}.
    Vprold,                           // Instruction 'vprold' {AVX512_F+VL}.
    Vprolq,                           // Instruction 'vprolq' {AVX512_F+VL}.
    Vprolvd,                          // Instruction 'vprolvd' {AVX512_F+VL}.
    Vprolvq,                          // Instruction 'vprolvq' {AVX512_F+VL}.
    Vprord,                           // Instruction 'vprord' {AVX512_F+VL}.
    Vprorq,                           // Instruction 'vprorq' {AVX512_F+VL}.
    Vprorvd,                          // Instruction 'vprorvd' {AVX512_F+VL}.
    Vprorvq,                          // Instruction 'vprorvq' {AVX512_F+VL}.
    Vprotb,                           // Instruction 'vprotb' {XOP}.
    Vprotd,                           // Instruction 'vprotd' {XOP}.
    Vprotq,                           // Instruction 'vprotq' {XOP}.
    Vprotw,                           // Instruction 'vprotw' {XOP}.
    Vpsadbw,                          // Instruction 'vpsadbw' {AVX|AVX2|AVX512_BW+VL}.
    Vpscatterdd,                      // Instruction 'vpscatterdd' {AVX512_F+VL}.
    Vpscatterdq,                      // Instruction 'vpscatterdq' {AVX512_F+VL}.
    Vpscatterqd,                      // Instruction 'vpscatterqd' {AVX512_F+VL}.
    Vpscatterqq,                      // Instruction 'vpscatterqq' {AVX512_F+VL}.
    Vpshab,                           // Instruction 'vpshab' {XOP}.
    Vpshad,                           // Instruction 'vpshad' {XOP}.
    Vpshaq,                           // Instruction 'vpshaq' {XOP}.
    Vpshaw,                           // Instruction 'vpshaw' {XOP}.
    Vpshlb,                           // Instruction 'vpshlb' {XOP}.
    Vpshld,                           // Instruction 'vpshld' {XOP}.
    Vpshldd,                          // Instruction 'vpshldd' {AVX512_VBMI2+VL}.
    Vpshldq,                          // Instruction 'vpshldq' {AVX512_VBMI2+VL}.
    Vpshldvd,                         // Instruction 'vpshldvd' {AVX512_VBMI2+VL}.
    Vpshldvq,                         // Instruction 'vpshldvq' {AVX512_VBMI2+VL}.
    Vpshldvw,                         // Instruction 'vpshldvw' {AVX512_VBMI2+VL}.
    Vpshldw,                          // Instruction 'vpshldw' {AVX512_VBMI2+VL}.
    Vpshlq,                           // Instruction 'vpshlq' {XOP}.
    Vpshlw,                           // Instruction 'vpshlw' {XOP}.
    Vpshrdd,                          // Instruction 'vpshrdd' {AVX512_VBMI2+VL}.
    Vpshrdq,                          // Instruction 'vpshrdq' {AVX512_VBMI2+VL}.
    Vpshrdvd,                         // Instruction 'vpshrdvd' {AVX512_VBMI2+VL}.
    Vpshrdvq,                         // Instruction 'vpshrdvq' {AVX512_VBMI2+VL}.
    Vpshrdvw,                         // Instruction 'vpshrdvw' {AVX512_VBMI2+VL}.
    Vpshrdw,                          // Instruction 'vpshrdw' {AVX512_VBMI2+VL}.
    Vpshufb,                          // Instruction 'vpshufb' {AVX|AVX2|AVX512_BW+VL}.
    Vpshufbitqmb,                     // Instruction 'vpshufbitqmb' {AVX512_BITALG+VL}.
    Vpshufd,                          // Instruction 'vpshufd' {AVX|AVX2|AVX512_F+VL}.
    Vpshufhw,                         // Instruction 'vpshufhw' {AVX|AVX2|AVX512_BW+VL}.
    Vpshuflw,                         // Instruction 'vpshuflw' {AVX|AVX2|AVX512_BW+VL}.
    Vpsignb,                          // Instruction 'vpsignb' {AVX|AVX2}.
    Vpsignd,                          // Instruction 'vpsignd' {AVX|AVX2}.
    Vpsignw,                          // Instruction 'vpsignw' {AVX|AVX2}.
    Vpslld,                           // Instruction 'vpslld' {AVX|AVX2|AVX512_F+VL}.
    Vpslldq,                          // Instruction 'vpslldq' {AVX|AVX2|AVX512_BW+VL}.
    Vpsllq,                           // Instruction 'vpsllq' {AVX|AVX2|AVX512_F+VL}.
    Vpsllvd,                          // Instruction 'vpsllvd' {AVX2|AVX512_F+VL}.
    Vpsllvq,                          // Instruction 'vpsllvq' {AVX2|AVX512_F+VL}.
    Vpsllvw,                          // Instruction 'vpsllvw' {AVX512_BW+VL}.
    Vpsllw,                           // Instruction 'vpsllw' {AVX|AVX2|AVX512_BW+VL}.
    Vpsrad,                           // Instruction 'vpsrad' {AVX|AVX2|AVX512_F+VL}.
    Vpsraq,                           // Instruction 'vpsraq' {AVX512_F+VL}.
    Vpsravd,                          // Instruction 'vpsravd' {AVX2|AVX512_F+VL}.
    Vpsravq,                          // Instruction 'vpsravq' {AVX512_F+VL}.
    Vpsravw,                          // Instruction 'vpsravw' {AVX512_BW+VL}.
    Vpsraw,                           // Instruction 'vpsraw' {AVX|AVX2|AVX512_BW+VL}.
    Vpsrld,                           // Instruction 'vpsrld' {AVX|AVX2|AVX512_F+VL}.
    Vpsrldq,                          // Instruction 'vpsrldq' {AVX|AVX2|AVX512_BW+VL}.
    Vpsrlq,                           // Instruction 'vpsrlq' {AVX|AVX2|AVX512_F+VL}.
    Vpsrlvd,                          // Instruction 'vpsrlvd' {AVX2|AVX512_F+VL}.
    Vpsrlvq,                          // Instruction 'vpsrlvq' {AVX2|AVX512_F+VL}.
    Vpsrlvw,                          // Instruction 'vpsrlvw' {AVX512_BW+VL}.
    Vpsrlw,                           // Instruction 'vpsrlw' {AVX|AVX2|AVX512_BW+VL}.
    Vpsubb,                           // Instruction 'vpsubb' {AVX|AVX2|AVX512_BW+VL}.
    Vpsubd,                           // Instruction 'vpsubd' {AVX|AVX2|AVX512_F+VL}.
    Vpsubq,                           // Instruction 'vpsubq' {AVX|AVX2|AVX512_F+VL}.
    Vpsubsb,                          // Instruction 'vpsubsb' {AVX|AVX2|AVX512_BW+VL}.
    Vpsubsw,                          // Instruction 'vpsubsw' {AVX|AVX2|AVX512_BW+VL}.
    Vpsubusb,                         // Instruction 'vpsubusb' {AVX|AVX2|AVX512_BW+VL}.
    Vpsubusw,                         // Instruction 'vpsubusw' {AVX|AVX2|AVX512_BW+VL}.
    Vpsubw,                           // Instruction 'vpsubw' {AVX|AVX2|AVX512_BW+VL}.
    Vpternlogd,                       // Instruction 'vpternlogd' {AVX512_F+VL}.
    Vpternlogq,                       // Instruction 'vpternlogq' {AVX512_F+VL}.
    Vptest,                           // Instruction 'vptest' {AVX}.
    Vptestmb,                         // Instruction 'vptestmb' {AVX512_BW+VL}.
    Vptestmd,                         // Instruction 'vptestmd' {AVX512_F+VL}.
    Vptestmq,                         // Instruction 'vptestmq' {AVX512_F+VL}.
    Vptestmw,                         // Instruction 'vptestmw' {AVX512_BW+VL}.
    Vptestnmb,                        // Instruction 'vptestnmb' {AVX512_BW+VL}.
    Vptestnmd,                        // Instruction 'vptestnmd' {AVX512_F+VL}.
    Vptestnmq,                        // Instruction 'vptestnmq' {AVX512_F+VL}.
    Vptestnmw,                        // Instruction 'vptestnmw' {AVX512_BW+VL}.
    Vpunpckhbw,                       // Instruction 'vpunpckhbw' {AVX|AVX2|AVX512_BW+VL}.
    Vpunpckhdq,                       // Instruction 'vpunpckhdq' {AVX|AVX2|AVX512_F+VL}.
    Vpunpckhqdq,                      // Instruction 'vpunpckhqdq' {AVX|AVX2|AVX512_F+VL}.
    Vpunpckhwd,                       // Instruction 'vpunpckhwd' {AVX|AVX2|AVX512_BW+VL}.
    Vpunpcklbw,                       // Instruction 'vpunpcklbw' {AVX|AVX2|AVX512_BW+VL}.
    Vpunpckldq,                       // Instruction 'vpunpckldq' {AVX|AVX2|AVX512_F+VL}.
    Vpunpcklqdq,                      // Instruction 'vpunpcklqdq' {AVX|AVX2|AVX512_F+VL}.
    Vpunpcklwd,                       // Instruction 'vpunpcklwd' {AVX|AVX2|AVX512_BW+VL}.
    Vpxor,                            // Instruction 'vpxor' {AVX|AVX2}.
    Vpxord,                           // Instruction 'vpxord' {AVX512_F+VL}.
    Vpxorq,                           // Instruction 'vpxorq' {AVX512_F+VL}.
    Vrangepd,                         // Instruction 'vrangepd' {AVX512_DQ+VL}.
    Vrangeps,                         // Instruction 'vrangeps' {AVX512_DQ+VL}.
    Vrangesd,                         // Instruction 'vrangesd' {AVX512_DQ}.
    Vrangess,                         // Instruction 'vrangess' {AVX512_DQ}.
    Vrcp14pd,                         // Instruction 'vrcp14pd' {AVX512_F+VL}.
    Vrcp14ps,                         // Instruction 'vrcp14ps' {AVX512_F+VL}.
    Vrcp14sd,                         // Instruction 'vrcp14sd' {AVX512_F}.
    Vrcp14ss,                         // Instruction 'vrcp14ss' {AVX512_F}.
    Vrcp28pd,                         // Instruction 'vrcp28pd' {AVX512_ERI}.
    Vrcp28ps,                         // Instruction 'vrcp28ps' {AVX512_ERI}.
    Vrcp28sd,                         // Instruction 'vrcp28sd' {AVX512_ERI}.
    Vrcp28ss,                         // Instruction 'vrcp28ss' {AVX512_ERI}.
    Vrcpps,                           // Instruction 'vrcpps' {AVX}.
    Vrcpss,                           // Instruction 'vrcpss' {AVX}.
    Vreducepd,                        // Instruction 'vreducepd' {AVX512_DQ+VL}.
    Vreduceps,                        // Instruction 'vreduceps' {AVX512_DQ+VL}.
    Vreducesd,                        // Instruction 'vreducesd' {AVX512_DQ}.
    Vreducess,                        // Instruction 'vreducess' {AVX512_DQ}.
    Vrndscalepd,                      // Instruction 'vrndscalepd' {AVX512_F+VL}.
    Vrndscaleps,                      // Instruction 'vrndscaleps' {AVX512_F+VL}.
    Vrndscalesd,                      // Instruction 'vrndscalesd' {AVX512_F}.
    Vrndscaless,                      // Instruction 'vrndscaless' {AVX512_F}.
    Vroundpd,                         // Instruction 'vroundpd' {AVX}.
    Vroundps,                         // Instruction 'vroundps' {AVX}.
    Vroundsd,                         // Instruction 'vroundsd' {AVX}.
    Vroundss,                         // Instruction 'vroundss' {AVX}.
    Vrsqrt14pd,                       // Instruction 'vrsqrt14pd' {AVX512_F+VL}.
    Vrsqrt14ps,                       // Instruction 'vrsqrt14ps' {AVX512_F+VL}.
    Vrsqrt14sd,                       // Instruction 'vrsqrt14sd' {AVX512_F}.
    Vrsqrt14ss,                       // Instruction 'vrsqrt14ss' {AVX512_F}.
    Vrsqrt28pd,                       // Instruction 'vrsqrt28pd' {AVX512_ERI}.
    Vrsqrt28ps,                       // Instruction 'vrsqrt28ps' {AVX512_ERI}.
    Vrsqrt28sd,                       // Instruction 'vrsqrt28sd' {AVX512_ERI}.
    Vrsqrt28ss,                       // Instruction 'vrsqrt28ss' {AVX512_ERI}.
    Vrsqrtps,                         // Instruction 'vrsqrtps' {AVX}.
    Vrsqrtss,                         // Instruction 'vrsqrtss' {AVX}.
    Vscalefpd,                        // Instruction 'vscalefpd' {AVX512_F+VL}.
    Vscalefps,                        // Instruction 'vscalefps' {AVX512_F+VL}.
    Vscalefsd,                        // Instruction 'vscalefsd' {AVX512_F}.
    Vscalefss,                        // Instruction 'vscalefss' {AVX512_F}.
    Vscatterdpd,                      // Instruction 'vscatterdpd' {AVX512_F+VL}.
    Vscatterdps,                      // Instruction 'vscatterdps' {AVX512_F+VL}.
    Vscatterpf0dpd,                   // Instruction 'vscatterpf0dpd' {AVX512_PFI}.
    Vscatterpf0dps,                   // Instruction 'vscatterpf0dps' {AVX512_PFI}.
    Vscatterpf0qpd,                   // Instruction 'vscatterpf0qpd' {AVX512_PFI}.
    Vscatterpf0qps,                   // Instruction 'vscatterpf0qps' {AVX512_PFI}.
    Vscatterpf1dpd,                   // Instruction 'vscatterpf1dpd' {AVX512_PFI}.
    Vscatterpf1dps,                   // Instruction 'vscatterpf1dps' {AVX512_PFI}.
    Vscatterpf1qpd,                   // Instruction 'vscatterpf1qpd' {AVX512_PFI}.
    Vscatterpf1qps,                   // Instruction 'vscatterpf1qps' {AVX512_PFI}.
    Vscatterqpd,                      // Instruction 'vscatterqpd' {AVX512_F+VL}.
    Vscatterqps,                      // Instruction 'vscatterqps' {AVX512_F+VL}.
    Vshuff32x4,                       // Instruction 'vshuff32x4' {AVX512_F+VL}.
    Vshuff64x2,                       // Instruction 'vshuff64x2' {AVX512_F+VL}.
    Vshufi32x4,                       // Instruction 'vshufi32x4' {AVX512_F+VL}.
    Vshufi64x2,                       // Instruction 'vshufi64x2' {AVX512_F+VL}.
    Vshufpd,                          // Instruction 'vshufpd' {AVX|AVX512_F+VL}.
    Vshufps,                          // Instruction 'vshufps' {AVX|AVX512_F+VL}.
    Vsqrtpd,                          // Instruction 'vsqrtpd' {AVX|AVX512_F+VL}.
    Vsqrtps,                          // Instruction 'vsqrtps' {AVX|AVX512_F+VL}.
    Vsqrtsd,                          // Instruction 'vsqrtsd' {AVX|AVX512_F}.
    Vsqrtss,                          // Instruction 'vsqrtss' {AVX|AVX512_F}.
    Vstmxcsr,                         // Instruction 'vstmxcsr' {AVX}.
    Vsubpd,                           // Instruction 'vsubpd' {AVX|AVX512_F+VL}.
    Vsubps,                           // Instruction 'vsubps' {AVX|AVX512_F+VL}.
    Vsubsd,                           // Instruction 'vsubsd' {AVX|AVX512_F}.
    Vsubss,                           // Instruction 'vsubss' {AVX|AVX512_F}.
    Vtestpd,                          // Instruction 'vtestpd' {AVX}.
    Vtestps,                          // Instruction 'vtestps' {AVX}.
    Vucomisd,                         // Instruction 'vucomisd' {AVX|AVX512_F}.
    Vucomiss,                         // Instruction 'vucomiss' {AVX|AVX512_F}.
    Vunpckhpd,                        // Instruction 'vunpckhpd' {AVX|AVX512_F+VL}.
    Vunpckhps,                        // Instruction 'vunpckhps' {AVX|AVX512_F+VL}.
    Vunpcklpd,                        // Instruction 'vunpcklpd' {AVX|AVX512_F+VL}.
    Vunpcklps,                        // Instruction 'vunpcklps' {AVX|AVX512_F+VL}.
    Vxorpd,                           // Instruction 'vxorpd' {AVX|AVX512_DQ+VL}.
    Vxorps,                           // Instruction 'vxorps' {AVX|AVX512_DQ+VL}.
    Vzeroall,                         // Instruction 'vzeroall' {AVX}.
    Vzeroupper,                       // Instruction 'vzeroupper' {AVX}.
    Wbinvd,                           // Instruction 'wbinvd'.
    Wbnoinvd,                         // Instruction 'wbnoinvd' {WBNOINVD}.
    Wrfsbase,                         // Instruction 'wrfsbase' {FSGSBASE} (X64).
    Wrgsbase,                         // Instruction 'wrgsbase' {FSGSBASE} (X64).
    Wrmsr,                            // Instruction 'wrmsr' {MSR}.
    Xabort,                           // Instruction 'xabort' {RTM}.
    Xadd,                             // Instruction 'xadd' {I486}.
    Xbegin,                           // Instruction 'xbegin' {RTM}.
    Xchg,                             // Instruction 'xchg'.
    Xend,                             // Instruction 'xend' {RTM}.
    Xgetbv,                           // Instruction 'xgetbv' {XSAVE}.
    Xlatb,                            // Instruction 'xlatb'.
    Xor,                              // Instruction 'xor'.
    Xorpd,                            // Instruction 'xorpd' {SSE2}.
    Xorps,                            // Instruction 'xorps' {SSE}.
    Xrstor,                           // Instruction 'xrstor' {XSAVE}.
    Xrstor64,                         // Instruction 'xrstor64' {XSAVE} (X64).
    Xrstors,                          // Instruction 'xrstors' {XSAVES}.
    Xrstors64,                        // Instruction 'xrstors64' {XSAVES} (X64).
    Xsave,                            // Instruction 'xsave' {XSAVE}.
    Xsave64,                          // Instruction 'xsave64' {XSAVE} (X64).
    Xsavec,                           // Instruction 'xsavec' {XSAVEC}.
    Xsavec64,                         // Instruction 'xsavec64' {XSAVEC} (X64).
    Xsaveopt,                         // Instruction 'xsaveopt' {XSAVEOPT}.
    Xsaveopt64,                       // Instruction 'xsaveopt64' {XSAVEOPT} (X64).
    Xsaves,                           // Instruction 'xsaves' {XSAVES}.
    Xsaves64,                         // Instruction 'xsaves64' {XSAVES} (X64).
    Xsetbv,                           // Instruction 'xsetbv' {XSAVE}.
    Xtest,                            // Instruction 'xtest' {TSX}.
}

/*
/// Instruction options.
enum Options {
    Vex3           = 0x00000400u, // Use 3-byte VEX prefix if possible (AVX) (must be 0x00000400).
    ModMR          = 0x00000800u, // Use ModMR instead of ModRM when it's available.
    Evex           = 0x00001000u, // Use 4-byte EVEX prefix if possible (AVX-512) (must be 0x00001000).

    Lock           = 0x00002000u, // LOCK prefix (lock-enabled instructions only).
    Rep            = 0x00004000u, // REP prefix (string instructions only).
    Repne          = 0x00008000u, // REPNE prefix (string instructions only).

    XAcquire       = 0x00010000u, // XACQUIRE prefix (only allowed instructions).
    XRelease       = 0x00020000u, // XRELEASE prefix (only allowed instructions).

    ER             = 0x00040000u, // AVX-512: embedded-rounding {er} and implicit {sae}.
    SAE            = 0x00080000u, // AVX-512: suppress-all-exceptions {sae}.
    RnSae          = 0x00000000u, // AVX-512: round-to-nearest (even)      {rn-sae} (bits 00).
    RdSae          = 0x00200000u, // AVX-512: round-down (toward -inf)     {rd-sae} (bits 01).
    RuSae          = 0x00400000u, // AVX-512: round-up (toward +inf)       {ru-sae} (bits 10).
    RzSae          = 0x00600000u, // AVX-512: round-toward-zero (truncate) {rz-sae} (bits 11).
    ZMask          = 0x00800000u, // AVX-512: Use zeroing {k}{z} instead of merging {k}.
    Avx512Mask     = 0x00FC0000u,  // AVX-512: Mask of all possible AVX-512 options except EVEX prefix flag.

    OpCodeB        = 0x01000000u, // REX.B and/or VEX.B field (X64).
    OpCodeX        = 0x02000000u, // REX.X and/or VEX.X field (X64).
    OpCodeR        = 0x04000000u, // REX.R and/or VEX.R field (X64).
    OpCodeW        = 0x08000000u, // REX.W and/or VEX.W field (X64).
    Rex            = 0x40000000u, // Force REX prefix (X64).
    InvalidRex     = 0x80000000u   // Invalid REX prefix (set by X86 or when AH|BH|CH|DH regs are used on X64).
}
*/

// CPP: changes 
// 1) unimplemented isDefinedId