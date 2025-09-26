//! Wasm-level "functions" used to implement Ruby semantics
//! TODO -- These should probably actually be real Wasm functions.
//!     Let Binaryen handle inlining.
//! TODO -- These would be more comprehensible if they used folded wat syntax
//!     But you'd need to explicitly take in args
//! TODO -- We would like to use IR ruby-values here.

use crate::{ArenaProvider, InstrSeqBuilder};
use crate::unitype::Unitype;
use crate::wasm::{BinaryOp, UnaryOp};

/// Is the given ruby-value equal to ruby-true?
/// `[Unitype] -> [I32]`
/// TODO -- We assume here that `Unitype ≡ I31`.
///     To do this generally, we need to cast to i31, and compare.
pub fn is_false<A: ArenaProvider>(builder: &mut InstrSeqBuilder<A>) {
    builder.unop(UnaryOp::I31GetU)
        .i32_const(Unitype::FALSE_BIT_PATTERN)
        .binop(BinaryOp::I32Eq);
}

// /// Is the given value ruby-truthy?
// /// All Ruby values are ruby-truthy, except for:
// /// - false
// /// - nil
// /// `[Unitype] -> [I32]`
// pub fn is_truthy() -> Vec<Instruction> {
//     // (not (or (is_nil $0) (is_false $0)))
//     // TODO -- Not possible to duplicate and (is_x ...) a single value
//     // let mut res = vec![];
//     // res.append(&mut is_nil()); // I32
//     // res.append(&mut is_false()); // I32
//     // res.push(Instruction::I32Or);
//     // res.push(Instruction::I32Not);
//     // res
//     todo!()
// }
