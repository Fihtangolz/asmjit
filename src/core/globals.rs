use thiserror::Error;

/// AsmJit error type 
pub type AsmJitError = Option<Error>;

/// AsmJit errors.
#[derive(Error, Debug)]
pub enum Error {
  /// Out of memory.
  #[error("Out of memory")]
  OutOfMemory,

  /// Invalid argument.
  #[error("Invalid argument")]
  InvalidArgument,

  /// Invalid state.
  ///
  /// If this error is returned it means that either you are doing something
  /// wrong or AsmJit caught itself by doing something wrong. This error should
  /// never be ignored.
  #[error("Invalid state")]
  InvalidState,

  /// Invalid or incompatible architecture.
  #[error("Invalid architecture")]
  InvalidArch,

  /// The object is not initialized.
  #[error("Not initialized")]
  NotInitialized,
  
  /// The object is already initialized.
  #[error("Already initialized")]
  AlreadyInitialized,

  /// Built-in feature was disabled at compile time and it's not available.
  #[error("Feature not enabled")]
  FeatureNotEnabled,

  /// Too many handles (Windows) or file descriptors (Unix/Posix).
  #[error("Too many handles or file descriptors")]
  TooManyHandles,
  
  /// Code generated is larger than allowed.
  #[error("Too large (code or memory request)")]
  TooLarge,

  /// No code generated.
  ///
  /// Returned by runtime if the `CodeHolder` contains no code.
  #[error("No code generated")]
  NoCodeGenerated,

  /// Invalid directive.
  #[error("Invalid directive")]
  InvalidDirective,
  
  /// Attempt to use uninitialized label.
  #[error("Invalid label")]
  InvalidLabel,

  /// Label index overflow - a single `Assembler` instance can hold almost
  /// 2^32 (4 billion) labels. If there is an attempt to create more labels
  /// then this error is returned.
  #[error("Too many labels")]
  TooManyLabels,

  /// Label is already bound.
  #[error("Label already bound")]
  LabelAlreadyBound,
  
  /// Label is already defined (named labels).
  #[error("Label already defined")]
  LabelAlreadyDefined,
  
  /// Label name is too long.
  #[error("Label name too long")]
  LabelNameTooLong,
  
  /// Label must always be local if it's anonymous (without a name).
  #[error("Invalid label name")]
  InvalidLabelName,
  
  /// Parent id passed to `CodeHolder::newNamedLabelId()` was invalid.
  #[error("Invalid parent label")]
  InvalidParentLabel,
  
  /// Parent id specified for a non-local (global) label.
  #[error("Non-local label can't have parent")]
  NonLocalLabelCantHaveParent,

  /// Invalid section.
  #[error("Invalid section")]
  InvalidSection,

  /// Too many sections (section index overflow).
  #[error("Too many sections")]
  TooManySections,

  /// Invalid section name (most probably too long).
  #[error("Invalid section name")]
  InvalidSectionName,

  /// Relocation index overflow (too many relocations).
  #[error("Too many relocations")]
  TooManyRelocations,
  
  /// Invalid relocation entry.
  #[error("Invalid relocation entry")]
  InvalidRelocEntry,
  
  /// Reloc entry contains address that is out of range (unencodable).
  #[error("Relocation offset out of range")]
  RelocOffsetOutOfRange,

  /// Invalid assignment to a register, function argument, or function return value.
  #[error("Invalid assignment")]
  InvalidAssignment,

  /// Invalid instruction.
  #[error("Invalid instruction")]
  InvalidInstruction,

  /// Invalid register type.
  #[error("Invalid register type")]
  InvalidRegType,

  /// Invalid register group.
  #[error("Invalid register group")]
  InvalidRegGroup,

  /// Invalid register's physical id.
  #[error("Invalid register physical id")]
  InvalidPhysId,

  /// Invalid register's virtual id.
  #[error("Invalid register virtual id")]
  InvalidVirtId,

  /// Invalid prefix combination.
  #[error("Invalid prefix combination")]
  InvalidPrefixCombination,

  /// Invalid LOCK prefix.
  #[error("Invalid lock prefix")]
  InvalidLockPrefix,

  /// Invalid XACQUIRE prefix.
  #[error("Invalid xacquire prefix")]
  InvalidXAcquirePrefix,

  /// Invalid XRELEASE prefix.
  #[error("Invalid xrelease prefix")]
  InvalidXReleasePrefix,

  /// Invalid REP prefix.
  #[error("Invalid rep prefix")]
  InvalidRepPrefix,

  /// Invalid REX prefix.
  #[error("Invalid rex prefix")]
  InvalidRexPrefix,

  /// Invalid {...} register.
  #[error("Invalid {...} register ")]
  InvalidExtraReg,

  /// Invalid {k} use (not supported by the instruction).
  #[error("Invalid use of {k}")]
  InvalidKMaskUse,

  /// Invalid {k}{z} use (not supported by the instruction).
  #[error("Invalid use of {k}{z}")]
  InvalidKZeroUse,

  /// Invalid broadcast - Currently only related to invalid use of AVX-512 {1tox}.
  #[error("Invalid broadcast {1tox}")]
  InvalidBroadcast,

  /// Invalid 'embedded-rounding' {er} or 'suppress-all-exceptions' {sae} (AVX-512).
  #[error("Invalid {er} or {sae} option")]
  InvalidEROrSAE,

  /// Invalid address used (not encodable).
  #[error("Invalid address")]
  InvalidAddress,

  /// Invalid index register used in memory address (not encodable).
  #[error("Invalid address index")]
  InvalidAddressIndex,

  /// Invalid address scale (not encodable).
  #[error("Invalid address scale")]
  InvalidAddressScale,
  
  /// Invalid use of 64-bit address.
  #[error("Invalid use of 64-bit address or offset")]
  InvalidAddress64Bit,
  
  /// Invalid use of 64-bit address that require 32-bit zero-extension (X64).
  #[error("Invalid use of 64-bit address or offset that requires 32-bit zero-extension")]
  InvalidAddress64BitZeroExtension,
  
  /// Invalid displacement (not encodable).
  #[error("Invalid displacement")]
  InvalidDisplacement,
  
  /// Invalid segment (X86).
  #[error("Invalid segment")]
  InvalidSegment,

  /// Invalid immediate (out of bounds on X86 and invalid pattern on ARM).
  #[error("Invalid immediate value")]
  InvalidImmediate,

  /// Invalid operand size.
  #[error("Invalid operand size")]
  InvalidOperandSize,

  /// Ambiguous operand size (memory has zero size while it's required to determine the operation type.
  #[error("Ambiguous operand size")]
  AmbiguousOperandSize,

  /// Mismatching operand size (size of multiple operands doesn't match the operation size).
  #[error("Operand size mismatch")]
  OperandSizeMismatch,

  /// Invalid option.
  #[error("Invalid option")]
  InvalidOption,

  /// Option already defined.
  #[error("Option already defined")]
  OptionAlreadyDefined,

  /// Invalid TypeId.
  #[error("Invalid type-info")]
  InvalidTypeId,

  /// Invalid use of a 8-bit GPB-HIGH register.
  #[error("Invalid use of a low 8-bit GPB register")]
  InvalidUseOfGpbHi,
  
  /// Invalid use of a 64-bit GPQ register in 32-bit mode.
  #[error("Invalid use of a 64-bit GPQ register in 32-bit mode")]
  InvalidUseOfGpq,
  
  /// Invalid use of an 80-bit float (Type::kIdF80).
  #[error("Invalid use of an 80-bit float")]
  InvalidUseOfF80,
  
  /// Some registers in the instruction muse be consecutive (some ARM and AVX512 neural-net instructions).
  #[error("Not consecutive registers")]
  NotConsecutiveRegs,

  /// AsmJit requires a physical register, but no one is available.
  #[error("No more physical registers")]
  NoMorePhysRegs,

  /// A variable has been assigned more than once to a function argument (BaseCompiler).
  #[error("Overlapped registers")]
  OverlappedRegs,

  /// Invalid register to hold stack arguments offset.
  #[error("Overlapping register and arguments base-address register")]
  OverlappingStackRegWithRegArg,

  /// Unbound label cannot be evaluated by expression.
  #[error("Unbound label cannot be evaluated by expression")]
  ExpressionLabelNotBound,
  
  /// Arithmetic overflow during expression evaluation.
  #[error("Arithmetic overflow during expression evaluation")]
  ExpressionOverflow,
}

// CPP: changes 
// 1) sucess error code removed, err type replased by Option<Error>
// 2) remove enum code number
// 3) Replace DebugUtils::errorAsString with thiserror
// 4) Remove DebugUtils::debugOutput (we have log)
// 5) Remove DebugUtils::assertionFailed (we have panic!)