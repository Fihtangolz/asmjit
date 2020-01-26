## [Unreleased] 
### Changed 
current porting fixed on commit [911a9a50a2cbc9802c4d44e94b8b99e1d9285cf3](https://github.com/asmjit/asmjit/tree/911a9a50a2cbc9802c4d44e94b8b99e1d9285cf3
- virtmem.[h/cpp]
    - Implementation moved to module, generic interface presented at mod.rs file. Platform specific implementations separait by files win/unix 
    - Comments refactoring
    - Inside fn info, atomic replased by std::sync::Once 
    - Scoped file classes replased by deffer
    - Static varible at file scope moved inside function 
- arch.[h/cpp]
    - signature from ArchInfo removed
    - ArchInfo::Id implemented like enum ArchVariants
    - isX86Family and isArmFamily moved to ArchVariants
    - is32Bit and is64Bit separated from main trait 
    - creation ArchRegs for host target implemented like function from_host in ArchInfo impl
    - reset was removed
    - init rename into new 