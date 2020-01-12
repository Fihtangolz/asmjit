//! TypeId.
//!
//! This is an additional information that can be used to describe a value-type
//! of physical or virtual register. it's used mostly by BaseCompiler to describe
//! register representation (the group of data stored in the register and the
//! width used) and it's also used by APIs that allow to describe and work with
//! function signatures.
pub enum Type {
    Void,
  
    BaseStart,
    BaseEnd,
  
    IntStart,
    IntEnd,
  
    IntPtr,
    UIntPtr,
  
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
  
    FloatStart,
    FloatEnd,
  
    F32,
    F64,
    F80,
  
    MaskStart,
    MaskEnd,
  
    Mask8,
    Mask16,
    Mask32,
    Mask64,
  
    MmxStart,
    MmxEnd,
  
    Mmx32,
    Mmx64,
  
    Vec32Start,
    Vec32End,
  
    I8x4,
    U8x4,
    I16x2,
    U16x2,
    I32x1,
    U32x1,
    F32x1,
  
    Vec64Start,
    Vec64End,
  
    I8x8,
    U8x8,
    I16x4,
    U16x4,
    I32x2,
    U32x2,
    I64x1,
    U64x1,
    F32x2,
    F64x1,
  
    Vec128Start,
    Vec128End,
  
    I8x16,
    U8x16,
    I16x8,
    U16x8,
    I32x4,
    U32x4,
    I64x2,
    U64x2,
    F32x4,
    F64x2,
  
    Vec256Start,
    Vec256End,
  
    I8x32,
    U8x32,
    I16x16,
    U16x16,
    I32x8,
    U32x8,
    I64x4,
    U64x4,
    F32x8,
    F64x4,
  
    Vec512Start,
    Vec512End,
  
    I8x64,
    U8x64,
    I16x32,
    U16x32,
    I32x16,
    U32x16,
    I64x8,
    U64x8,
    F32x16,
    F64x8,
}