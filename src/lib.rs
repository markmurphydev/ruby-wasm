pub mod lexeme;
pub mod lexer;
pub mod node;
pub mod parser;
pub mod wasm;
pub mod compiler;
pub mod binary;
pub mod html;
pub mod runtime;
mod arena_set;
pub mod run;

pub use crate::wasm::wat;
pub use crate::wasm::function::{ArenaProvider, FunctionBuilder, InstrSeqBuilder};
