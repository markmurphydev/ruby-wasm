//! Wasm -> .wasm binary printer
//! Currently goes Wasm -> .wat -> .wasm (with `wat` library)

use crate::wasm::Module;
use crate::wat::WatPrinter;

/// Convert the given Wasm module to its binary representation.
pub fn module_to_binary(module: &Module) -> Vec<u8> {
    let wat = WatPrinter::new().print_module(module);
    let binary = wat::parse_str(wat).unwrap();
    binary
}
