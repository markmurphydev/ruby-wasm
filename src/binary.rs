//! Wasm -> .wasm binary printer
//! Currently goes Wasm -> .wat -> .wasm (with `wat` library)

use std::io::Write;
use wat_defs::module::Module;
use crate::print_wat::module_to_pretty;

/// Convert the given Wasm module to its binary representation.
pub fn module_to_binary(module: &Module) -> Vec<u8> {
    let wat = module_to_pretty(module);
    let binary = wat::parse_str(wat).unwrap();
    binary
}

/// Write the given slice of `u8` to stdout.
pub fn print_bytes(bytes: &[u8]) {
    let mut stdout = std::io::stdout();
    stdout.write_all(bytes).unwrap();
    stdout.flush().unwrap();
}
