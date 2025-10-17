//! Wasm -> .wasm binary printer
//! Currently goes Wasm -> .wat -> .wasm (with `wat` library)

// use crate::wasm::module::Module;
// use std::io::Write;
// 
// /// Convert the given Wasm module to its binary representation.
// pub fn module_to_binary(module: &Module) -> Vec<u8> {
//     let wat = module.to_pretty();
//     let binary = wat::parse_str(wat).unwrap();
//     binary
// }
// 
// /// Write the given slice of `u8` to stdout.
// pub fn print_bytes(bytes: &[u8]) {
//     let mut stdout = std::io::stdout();
//     stdout.write_all(bytes).unwrap();
//     stdout.flush().unwrap();
// }
