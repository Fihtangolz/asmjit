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