// use std::fmt::{Display, Formatter};
//
// pub struct Byte(pub u8);
//
// /// Signed 32-bit integer
// #[derive(Debug, Copy, Clone)]
// pub struct U32(pub u32);
//
// /// Ambiguously-signed 32-bit integer
// /// TODO -- Currently we always treat as signed
// #[derive(Debug, Copy, Clone)]
// pub struct I32(pub u32);
//
// impl I32 {
//     // Constant data types are represented as `i31` tagged pointers.
//     // TODO -- This is not the final resting place of these.
//     pub const FALSE: I32 = I32(0b0001);
//     pub const TRUE: I32 = I32(0b0011);
//     pub const NIL: I32 = I32(0b0101);
// }
//
// impl Display for I32 {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         self.0.fmt(f)
//     }
// }
//
// #[derive(Debug, Copy, Clone)]
// pub struct I64(pub u64);
//
// impl Display for I64 {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         self.0.fmt(f)
//     }
// }

use crate::wasm::InstrSeq;
use crate::wasm::intern::InternedIdentifier;

/// A WAT function definition.
pub struct Function {
    name: InternedIdentifier,
    instr_seq: InstrSeq,
}
