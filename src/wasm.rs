//! A representation of the _compiled_ wasm program.
//! Should be trivially convertible to .wat
//! NB: Strange naming conventions are mostly copied from Wasm spec.

pub mod types;
pub mod values;

use crate::wasm::types::{GlobalType, Type, ValueType};
use crate::wasm::values::{I32, I64, U32};

// ==== Wasm Instructions ====

#[derive(Debug, Clone)]
pub enum Instruction {
    // Number instructions
    ConstI32(I32),
    ConstI64(I64),
    /// Convert an `i32` to a `(ref i31)`
    RefI31,
    /// Convert a `(ref i31)` to `i32`, treating as unsigned
    I31GetU,

    // Global instructions
    GlobalGet(GlobalIdx),

    I32Xor,
    I32Or,
    I32Eqz,
    I32Eq,

    // Control instructions
    If(If),
    Loop(Loop),
}

#[derive(Debug, Clone)]
/// (if label block_type? predicate_instrs* (then then_instrs*) (else else_instrs*)?)
pub struct If {
    /// Idk. Is it for named breaks?
    pub label: Option<String>,

    /// The return type of the if, else blocks
    /// TODO: This should be a union of something and valtype
    /// TODO: This might always be Unitype, or might sometimes be Unitype, sometimes Void
    pub block_type: Type,

    pub predicate_instrs: Vec<Instruction>,

    pub then_instrs: Vec<Instruction>,
    pub else_instrs: Vec<Instruction>,
}

#[derive(Debug, Clone)]
pub struct Loop {
    pub label: Option<String>,
    /// The return type of the loop instructions
    pub block_type: Type,
    pub instructions: Vec<Instruction>,
}

// ==== Wasm Module Items ====

#[derive(Debug, Clone)]
pub enum FunctionIdx {
    // TODO -- Spec defines indices to be wasm-u32
    // https://webassembly.github.io/spec/core/syntax/modules.html#syntax-start
    Index(U32),
    Id(String),
}

#[derive(Debug, Clone)]
pub struct Function {
    pub id: Option<String>,
    pub body: Expr,
}

/// Sequence of instructions
/// https://webassembly.github.io/spec/core/syntax/instructions.html#syntax-expr
#[derive(Debug, Clone)]
pub struct Expr(pub Vec<Instruction>);

#[derive(Debug, Clone)]
pub enum GlobalIdx {
    Idx(U32),
    Id(String),
}

#[derive(Debug, Clone)]
pub struct Global {
    pub id: Option<String>,
    pub global_type: GlobalType,
    pub expr: Expr,
}

#[derive(Debug, Clone)]
pub struct Module {
    pub functions: Vec<Function>,
    pub exports: Vec<FunctionIdx>,
    pub globals: Vec<Global>,
    /// A function `() -> ()` which _initializes_ the wasm module
    /// NB: _not_ a main function
    pub start: Option<FunctionIdx>,
}
