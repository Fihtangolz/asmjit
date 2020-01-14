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
    #[error("Invalid use of {{k}}")]
    InvalidKMaskUse,

    /// Invalid {k}{z} use (not supported by the instruction).
    #[error("Invalid use of {{k}}{{z}}")]
    InvalidKZeroUse,

    /// Invalid broadcast - Currently only related to invalid use of AVX-512 {1tox}.
    #[error("Invalid broadcast {{1tox}}")]
    InvalidBroadcast,

    /// Invalid 'embedded-rounding' {er} or 'suppress-all-exceptions' {sae} (AVX-512).
    #[error("Invalid {{er}} or {{sae}} option")]
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

/// Contains typedefs, constants, and variables used globally by AsmJit.
mod globals {

    // ============================================================================
    // [asmjit::Globals::<global>]
    // ============================================================================

    use std::mem;

    /// Host memory allocator overhead.
    pub const kAllocOverhead: usize = mem::size_of::<isize>() * 4;

    /// Host memory allocator alignment.
    pub const kAllocAlignment: u32 = 8;

    /// Aggressive growing strategy threshold.
    pub const kGrowThreshold: u32 = 1024 * 1024 * 16;

    /// Maximum height of RB-Tree is:
    ///
    ///   `2 * log2(n + 1)`.
    ///
    /// Size of RB node is at least two pointers (without data),
    /// so a theoretical architecture limit would be:
    ///
    ///   `2 * log2(addressableMemorySize / sizeof(Node) + 1)`
    ///
    /// Which yields 30 on 32-bit arch and 61 on 64-bit arch.
    /// The final value was adjusted by +1 for safety reasons.
    // TODO: const xMaxTreeHeight: usize = (ASMJIT_ARCH_BITS == 32 ? 30 : 61) + 1;


    /// Maximum number of operands per a single instruction.
    pub const kMaxOpCount: u32 = 6;

    // TODO: Use this one.
    pub const kMaxFuncArgs: u32 = 16;

    /// Maximum number of physical registers AsmJit can use per register group.
    pub const kMaxPhysRegs: u32 = 32;

    /// Maximum alignment.
    pub const kMaxAlignment: u32 = 64;

    /// Maximum label or symbol size in bytes.
    pub const kMaxLabelNameSize: u32 = 2048;

    /// Maximum section name size.
    pub const kMaxSectionNameSize: u32 = 35;

    /// Maximum size of comment.
    pub const kMaxCommentSize: u32 = 1024;

    /// Invalid identifier.
    pub const kInvalidId: u32 = 0xFFFFFFFF;

    /// Returned by `indexOf()` and similar when working with containers that use 32-bit index/size.
    pub const kNotFound: u32 = 0xFFFFFFFF;

    /// Invalid base address.
    pub const kNoBaseAddress: u64 = (~0);

    // ============================================================================
    // [asmjit::Globals::ResetPolicy]
    // ============================================================================

    /// Reset policy used by most `reset()` functions.
    #[repr(u32)]
    pub enum ResetPolicy {
        /// Soft reset, doesn't deallocate memory (default).
        kResetSoft = 0,
        /// Hard reset, releases all memory used, if any.
        kResetHard = 1
    }

    // ============================================================================
    // [asmjit::Globals::Link]
    // ============================================================================

    pub const kLinkLeft: u32  = 0;
    pub const kLinkRight: u32 = 1;

    pub const kLinkPrev: u32  = 0;
    pub const kLinkNext: u32  = 1;

    pub const kLinkFirst: u32 = 0;
    pub const kLinkLast: u32  = 1;

    pub const kLinkCount: u32 = 2;

    // ============================================================================
    // [asmjit::ByteOrder]
    // ============================================================================

    #[repr(u32)]
    pub enum ByteOrder {
        kLE = 0,
        kBE = 1,
        // TODO: kNative = ASMJIT_ARCH_LE ? kLE : kBE,
        // TODO: ASMJIT_ARCH_LE ? kBE : kLE,
    }
}

// CPP: changes
// 1) sucess error code removed, err type replased by Option<Error>
// 2) remove enum code number
// 3) Replace DebugUtils::errorAsString with thiserror
// 4) Remove DebugUtils::debugOutput (we have log)
// 5) Remove DebugUtils::assertionFailed (we have panic!)
