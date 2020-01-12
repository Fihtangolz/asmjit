/// AsmJit error type 
type AsmJitError = Option<Error>;

//! AsmJit errors
enum Error {
  /// Out of memory.
  OutOfMemory,
  /// Invalid argument.
  InvalidArgument,

  /// Invalid state.
  ///
  /// If this error is returned it means that either you are doing something
  /// wrong or AsmJit caught itself by doing something wrong. This error should
  /// never be ignored.
  InvalidState,

  /// Invalid or incompatible architecture.
  InvalidArch,

  /// The object is not initialized.
  NotInitialized,
  /// The object is already initialized.
  AlreadyInitialized,

  /// Built-in feature was disabled at compile time and it's not available.
  FeatureNotEnabled,

  /// Too many handles (Windows) or file descriptors (Unix/Posix).
  TooManyHandles,
  /// Code generated is larger than allowed.
  TooLarge,

  /// No code generated.
  ///
  /// Returned by runtime if the `CodeHolder` contains no code.
  NoCodeGenerated,

  /// Invalid directive.
  InvalidDirective,
  /// Attempt to use uninitialized label.
  InvalidLabel,
  /// Label index overflow - a single `Assembler` instance can hold almost
  /// 2^32 (4 billion) labels. If there is an attempt to create more labels
  /// then this error is returned.
  TooManyLabels,
  /// Label is already bound.
  LabelAlreadyBound,
  /// Label is already defined (named labels).
  LabelAlreadyDefined,
  /// Label name is too long.
  LabelNameTooLong,
  /// Label must always be local if it's anonymous (without a name).
  InvalidLabelName,
  /// Parent id passed to `CodeHolder::newNamedLabelId()` was invalid.
  InvalidParentLabel,
  /// Parent id specified for a non-local (global) label.
  NonLocalLabelCantHaveParent,

  /// Invalid section.
  InvalidSection,
  /// Too many sections (section index overflow).
  TooManySections,
  /// Invalid section name (most probably too long).
  InvalidSectionName,

  /// Relocation index overflow (too many relocations).
  TooManyRelocations,
  /// Invalid relocation entry.
  InvalidRelocEntry,
  /// Reloc entry contains address that is out of range (unencodable).
  RelocOffsetOutOfRange,

  /// Invalid assignment to a register, function argument, or function return value.
  InvalidAssignment,
  /// Invalid instruction.
  InvalidInstruction,
  /// Invalid register type.
  InvalidRegType,
  /// Invalid register group.
  InvalidRegGroup,
  /// Invalid register's physical id.
  InvalidPhysId,
  /// Invalid register's virtual id.
  InvalidVirtId,
  /// Invalid prefix combination.
  InvalidPrefixCombination,
  /// Invalid LOCK prefix.
  InvalidLockPrefix,
  /// Invalid XACQUIRE prefix.
  InvalidXAcquirePrefix,
  /// Invalid XRELEASE prefix.
  InvalidXReleasePrefix,
  /// Invalid REP prefix.
  InvalidRepPrefix,
  /// Invalid REX prefix.
  InvalidRexPrefix,
  /// Invalid {...} register.
  InvalidExtraReg,
  /// Invalid {k} use (not supported by the instruction).
  InvalidKMaskUse,
  /// Invalid {k}{z} use (not supported by the instruction).
  InvalidKZeroUse,
  /// Invalid broadcast - Currently only related to invalid use of AVX-512 {1tox}.
  InvalidBroadcast,
  /// Invalid 'embedded-rounding' {er} or 'suppress-all-exceptions' {sae} (AVX-512).
  InvalidEROrSAE,
  /// Invalid address used (not encodable).
  InvalidAddress,
  /// Invalid index register used in memory address (not encodable).
  InvalidAddressIndex,
  /// Invalid address scale (not encodable).
  InvalidAddressScale,
  /// Invalid use of 64-bit address.
  InvalidAddress64Bit,
  /// Invalid use of 64-bit address that require 32-bit zero-extension (X64).
  InvalidAddress64BitZeroExtension,
  /// Invalid displacement (not encodable).
  InvalidDisplacement,
  /// Invalid segment (X86).
  InvalidSegment,

  /// Invalid immediate (out of bounds on X86 and invalid pattern on ARM).
  InvalidImmediate,

  /// Invalid operand size.
  InvalidOperandSize,
  /// Ambiguous operand size (memory has zero size while it's required to determine the operation type.
  AmbiguousOperandSize,
  /// Mismatching operand size (size of multiple operands doesn't match the operation size).
  OperandSizeMismatch,

  /// Invalid option.
  InvalidOption,
  /// Option already defined.
  OptionAlreadyDefined,

  /// Invalid TypeId.
  InvalidTypeId,
  /// Invalid use of a 8-bit GPB-HIGH register.
  InvalidUseOfGpbHi,
  /// Invalid use of a 64-bit GPQ register in 32-bit mode.
  InvalidUseOfGpq,
  /// Invalid use of an 80-bit float (Type::kIdF80).
  InvalidUseOfF80,
  /// Some registers in the instruction muse be consecutive (some ARM and AVX512 neural-net instructions).
  NotConsecutiveRegs,

  /// AsmJit requires a physical register, but no one is available.
  NoMorePhysRegs,
  /// A variable has been assigned more than once to a function argument (BaseCompiler).
  OverlappedRegs,
  /// Invalid register to hold stack arguments offset.
  OverlappingStackRegWithRegArg,

  /// Unbound label cannot be evaluated by expression.
  ExpressionLabelNotBound,
  /// Arithmetic overflow during expression evaluation.
  ExpressionOverflow,
};

// CPP: changes 
// 1) sucess error code removed, err type replased by Option<Error>
// 2) remove enum code number 