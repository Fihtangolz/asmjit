struct JitRuntime {

}

/// JIT execution runtime is a special `Target` that is designed to store and
/// execute the generated code.
impl JitRuntime {
    aloc: JitAllocator;
    
    pub fn reset();
    pub fn aloc();
    pub fn add();
    pub fn release();
    pub fn flush();
}