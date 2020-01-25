use super::target::Target;
use super::jitallocator::JitAllocator;

/// JIT execution runtime is a special `Target` that is designed to store and
/// execute the generated code.
struct JitRuntime {
    aloc: JitAllocator,
    target: Target,
}

impl JitRuntime { 
    pub fn reset();
    pub fn aloc();
    pub fn add();
    pub fn release();
    pub fn flush();
}