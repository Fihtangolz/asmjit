//! AsmJit error type (uint32_t).
typedef uint32_t Error;

//! AsmJit error codes.
enum ErrorCode : uint32_t {
  //! No error (success).
  kErrorOk = 0,

  //! Out of memory.
  kErrorOutOfMemory,

  //! Invalid argument.
  kErrorInvalidArgument,

  //! Invalid state.
  //!
  //! If this error is returned it means that either you are doing something
  //! wrong or AsmJit caught itself by doing something wrong. This error should
  //! never be ignored.
  kErrorInvalidState,

  //! Invalid or incompatible architecture.
  kErrorInvalidArch,

  //! The object is not initialized.
  kErrorNotInitialized,
  //! The object is already initialized.
  kErrorAlreadyInitialized,

  //! Built-in feature was disabled at compile time and it's not available.
  kErrorFeatureNotEnabled,

  //! Too many handles (Windows) or file descriptors (Unix/Posix).
  kErrorTooManyHandles,
  //! Code generated is larger than allowed.
  kErrorTooLarge,

  //! No code generated.
  //!
  //! Returned by runtime if the `CodeHolder` contains no code.
  kErrorNoCodeGenerated,

  //! Invalid directive.
  kErrorInvalidDirective,
  //! Attempt to use uninitialized label.
  kErrorInvalidLabel,
  //! Label index overflow - a single `Assembler` instance can hold almost
  //! 2^32 (4 billion) labels. If there is an attempt to create more labels
  //! then this error is returned.
  kErrorTooManyLabels,
  //! Label is already bound.
  kErrorLabelAlreadyBound,
  //! Label is already defined (named labels).
  kErrorLabelAlreadyDefined,
  //! Label name is too long.
  kErrorLabelNameTooLong,
  //! Label must always be local if it's anonymous (without a name).
  kErrorInvalidLabelName,
  //! Parent id passed to `CodeHolder::newNamedLabelId()` was invalid.
  kErrorInvalidParentLabel,
  //! Parent id specified for a non-local (global) label.
  kErrorNonLocalLabelCantHaveParent,

  //! Invalid section.
  kErrorInvalidSection,
  //! Too many sections (section index overflow).
  kErrorTooManySections,
  //! Invalid section name (most probably too long).
  kErrorInvalidSectionName,

  //! Relocation index overflow (too many relocations).
  kErrorTooManyRelocations,
  //! Invalid relocation entry.
  kErrorInvalidRelocEntry,
  //! Reloc entry contains address that is out of range (unencodable).
  kErrorRelocOffsetOutOfRange,

  //! Invalid assignment to a register, function argument, or function return value.
  kErrorInvalidAssignment,
  //! Invalid instruction.
  kErrorInvalidInstruction,
  //! Invalid register type.
  kErrorInvalidRegType,
  //! Invalid register group.
  kErrorInvalidRegGroup,
  //! Invalid register's physical id.
  kErrorInvalidPhysId,
  //! Invalid register's virtual id.
  kErrorInvalidVirtId,
  //! Invalid prefix combination.
  kErrorInvalidPrefixCombination,
  //! Invalid LOCK prefix.
  kErrorInvalidLockPrefix,
  //! Invalid XACQUIRE prefix.
  kErrorInvalidXAcquirePrefix,
  //! Invalid XRELEASE prefix.
  kErrorInvalidXReleasePrefix,
  //! Invalid REP prefix.
  kErrorInvalidRepPrefix,
  //! Invalid REX prefix.
  kErrorInvalidRexPrefix,
  //! Invalid {...} register.
  kErrorInvalidExtraReg,
  //! Invalid {k} use (not supported by the instruction).
  kErrorInvalidKMaskUse,
  //! Invalid {k}{z} use (not supported by the instruction).
  kErrorInvalidKZeroUse,
  //! Invalid broadcast - Currently only related to invalid use of AVX-512 {1tox}.
  kErrorInvalidBroadcast,
  //! Invalid 'embedded-rounding' {er} or 'suppress-all-exceptions' {sae} (AVX-512).
  kErrorInvalidEROrSAE,
  //! Invalid address used (not encodable).
  kErrorInvalidAddress,
  //! Invalid index register used in memory address (not encodable).
  kErrorInvalidAddressIndex,
  //! Invalid address scale (not encodable).
  kErrorInvalidAddressScale,
  //! Invalid use of 64-bit address.
  kErrorInvalidAddress64Bit,
  //! Invalid use of 64-bit address that require 32-bit zero-extension (X64).
  kErrorInvalidAddress64BitZeroExtension,
  //! Invalid displacement (not encodable).
  kErrorInvalidDisplacement,
  //! Invalid segment (X86).
  kErrorInvalidSegment,

  //! Invalid immediate (out of bounds on X86 and invalid pattern on ARM).
  kErrorInvalidImmediate,

  //! Invalid operand size.
  kErrorInvalidOperandSize,
  //! Ambiguous operand size (memory has zero size while it's required to determine the operation type.
  kErrorAmbiguousOperandSize,
  //! Mismatching operand size (size of multiple operands doesn't match the operation size).
  kErrorOperandSizeMismatch,

  //! Invalid option.
  kErrorInvalidOption,
  //! Option already defined.
  kErrorOptionAlreadyDefined,

  //! Invalid TypeId.
  kErrorInvalidTypeId,
  //! Invalid use of a 8-bit GPB-HIGH register.
  kErrorInvalidUseOfGpbHi,
  //! Invalid use of a 64-bit GPQ register in 32-bit mode.
  kErrorInvalidUseOfGpq,
  //! Invalid use of an 80-bit float (Type::kIdF80).
  kErrorInvalidUseOfF80,
  //! Some registers in the instruction muse be consecutive (some ARM and AVX512 neural-net instructions).
  kErrorNotConsecutiveRegs,

  //! AsmJit requires a physical register, but no one is available.
  kErrorNoMorePhysRegs,
  //! A variable has been assigned more than once to a function argument (BaseCompiler).
  kErrorOverlappedRegs,
  //! Invalid register to hold stack arguments offset.
  kErrorOverlappingStackRegWithRegArg,

  //! Unbound label cannot be evaluated by expression.
  kErrorExpressionLabelNotBound,
  //! Arithmetic overflow during expression evaluation.
  kErrorExpressionOverflow,

  //! Count of AsmJit error codes.
  kErrorCount
};