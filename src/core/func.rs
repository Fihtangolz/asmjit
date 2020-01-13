use super::callconv::CallConvVariants;
use super::r#type;

/// Function detail - CallConv and expanded FuncSignature.
///
/// Function detail is architecture and OS dependent representation of a function.
/// It contains calling convention and expanded function signature so all
/// arguments have assigned either register type & id or stack address.
pub struct FuncDetail {
    call_conv: CallConvVariants,
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
    call_conv: CallConvVariants,
    /// Count of arguments.
    arg_count: u8,
    /// Index of a first VA or `kNoVarArgs`.
    va_index: u8,
    /// Return value TypeId.
    ret: r#type::Type,
    /// Function arguments TypeIds.
    args: r#type::Type,

    // enum : uint8_t {
    //     //! Doesn't have variable number of arguments (`...`).
    //     kNoVarArgs = 0xFF
    // };
}

impl FuncSignature {
    /// Initializes the function signature.
    pub fn init(&mut self, call_conv: CallConvVariants, va_index: u8, ret: r#type::Type, args: r#type::Type, arg_count: u8) {
        self.call_conv = call_conv;
        self.arg_count = arg_count;
        self.va_index = va_index;
        self.ret = ret;
        self.args = args;
    }
  
    pub fn reset(&mut self) {
        unimplemented!();
    }

    /// Returns the calling convention.
    pub fn call_conv(&self) -> &CallConvVariants { 
        &self.call_conv
    }
    /// Sets the calling convention to `ccId`;
    pub fn set_call_conv(&mut self, call_conv: CallConvVariants) { 
        self.call_conv = call_conv
    }
    /// Tests whether the function has variable number of arguments (...).
    pub fn has_var_args(&self) -> bool { 
        unimplemented!();
    }
    /// Returns the variable arguments (...) index, `kNoVarArgs` if none.
    pub fn va_index(&self) -> u8 { 
        self.va_index
    }
    /// Sets the variable arguments (...) index to `index`.
    pub fn set_va_index(&mut self, index: u8) { 
        self.va_index = index
    }
    /// Resets the variable arguments index (making it a non-va function).
    pub fn reset_va_index(&self) { 
        unimplemented!();
    }

    /// Returns the number of function arguments.
    pub fn arg_count(&self) -> u8 { 
        self.arg_count
    }
    
    pub fn has_ret(&self) -> bool { 
        self.ret != r#type::Type::Void
    }
    
    /// Returns the return value type.
    pub fn ret(&self) -> &r#type::Type { 
        &self.ret
    }

    /// Returns the type of the argument at index `i`.
    pub fn arg(&self, index: u8) -> &r#type::Type {
        if index > self.arg_count {
            panic!("out of range");            
        }
    
        &self.args
    }
}