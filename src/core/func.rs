mod callconv;

/// Function detail - CallConv and expanded FuncSignature.
///
/// Function detail is architecture and OS dependent representation of a function.
/// It contains calling convention and expanded function signature so all
/// arguments have assigned either register type & id or stack address.
pub struct FuncDetail {
    call_conv: callconv::CallConv,
    arg_count: u8,
    ret_count: u8,
}

impl FuncDetail {

}

/// Function frame.
///
/// Function frame is used directly by prolog and epilog insertion (PEI) utils.
/// It provides information necessary to insert a proper and ABI comforming
/// prolog and epilog. Function frame calculation is based on `CallConv` and
/// other function attributes.
///
/// Function Frame Structure
/// ------------------------
///
/// Various properties can contribute to the size and structure of the function
/// frame. The function frame in most cases won't use all of the properties
/// illustrated (for example Spill Zone and Red Zone are never used together).
///
///   +-----------------------------+
///   | Arguments Passed by Stack   |
///   +-----------------------------+
///   | Spill Zone                  |
///   +-----------------------------+ <- Stack offset (args) starts from here.
///   | Return Address if Pushed    |
///   +-----------------------------+ <- Stack pointer (SP) upon entry.
///   | Save/Restore Stack.         |
///   +-----------------------------+-----------------------------+
///   | Local Stack                 |                             |
///   +-----------------------------+          Final Stack        |
///   | Call Stack                  |                             |
///   +-----------------------------+-----------------------------+ <- SP after prolog.
///   | Red Zone                    |
///   +-----------------------------+
struct FuncFrame {
    
}

/// Function signature.
///
/// Contains information about function return type, count of arguments and
/// their TypeIds. Function signature is a low level structure which doesn't
/// contain platform specific or calling convention specific information.
struct FuncSignature {
    /// Calling convention id.
    call_сonv: CallConvVariants,
    /// Count of arguments.
    arg_count: u8,
    /// Index of a first VA or `kNoVarArgs`.
    va_index: u8,
    /// Return value TypeId.
    ret: u8,
    /// Function arguments TypeIds.
    const uint8_t* _args;

    enum : uint8_t {
        //! Doesn't have variable number of arguments (`...`).
        kNoVarArgs = 0xFF
    };
}

impl FuncSignature {
    /// Initializes the function signature.
    pub fn init(&mut self, call_сonv: CallConvVariants, va_index: u8, uint32_t ret, const uint8_t* args, arg_count: u8) {
        self.call_conv = call_сonv;
        self.arg_count = arg_count;
        self.va_index = va_index;
        self.ret = ret;
        self.args = args;
    }
  
    pub fn reset() {
        unimplemented!();
    }

    /// Returns the calling convention.
    pub fn call_conv(&self) -> CallConvVariants { 
        self.call_сonv
    }
    /// Sets the calling convention to `ccId`;
    pub fn set_call_conv(&mut self, call_сonv: CallConvVariants { 
        self.call_сonv = call_сonv;
    }
    /// Tests whether the function has variable number of arguments (...).
    pub fn has_var_args() -> bool { 
        return _vaIndex != kNoVarArgs; 
    }
    /// Returns the variable arguments (...) index, `kNoVarArgs` if none.
    inline uint32_t va_index() const noexcept { 
      return _vaIndex; 
    }
    /// Sets the variable arguments (...) index to `index`.
    inline void set_va_index(uint32_t index) noexcept { 
        _vaIndex = uint8_t(index); 
    }
    /// Resets the variable arguments index (making it a non-va function).
    inline void reset_va_index() noexcept { 
      _vaIndex = kNoVarArgs; 
    }

    /// Returns the number of function arguments.
    pub fn arg_count() -> u8 { 
        self.arg_count
    }

    pub fn has_ret(&self) -> bool { 
      return _ret != Type::kIdVoid; 
    }
    /// Returns the return value type.
    inline uint32_t ret() const noexcept { 
      return _ret; 
    }

    /// Returns the type of the argument at index `i`.
    inline uint32_t arg(uint32_t i) const noexcept {
        ASMJIT_ASSERT(i < _argCount);
        return _args[i];
    }
}