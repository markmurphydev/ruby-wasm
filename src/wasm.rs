//! A representation of the _compiled_ wasm program.
//! Should be trivially convertible to .wat

pub mod values;

pub use values::Integer;

// ==== Wasm Types ====

#[derive(Copy, Clone)]
pub enum Instruction {
    ConstI32(Integer),
    /// Convert an `i32` to a `(ref i31)`
    RefI31,
}

pub enum FunctionIndex {
    // TODO -- Spec defines indices to be wasm-u32
    // https://webassembly.github.io/spec/core/syntax/modules.html#syntax-start
    Index(usize),
    Name(String),
}

pub struct Function {
    pub name: Option<String>,
    pub body: Expr,
}

/// Sequence of instructions terminated by an `end` marker
/// https://webassembly.github.io/spec/core/syntax/instructions.html#syntax-expr
pub struct Expr(pub Vec<Instruction>);

pub struct Module {
    pub functions: Vec<Function>,
    pub exports: Vec<FunctionIndex>,
    /// A function `() -> ()` which _initializes_ the wasm module
    /// NB: _not_ a main function
    pub start: Option<FunctionIndex>,
}
