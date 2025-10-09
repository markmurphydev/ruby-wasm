pub mod binary;
pub mod compiler;
pub mod core;
pub mod html;
pub mod lexeme;
pub mod lexer;
pub mod node;
pub mod parser;
pub mod run;
pub mod unitype;
pub mod wasm;

pub use crate::compiler::CompileCtx;
pub use crate::wasm::function::FunctionBuilder;
pub use crate::wasm::wat;
pub use wasm::instr_seq::InstrSeqBuilder;

use crate::core::add_core_items;
use crate::lexer::Lexer;
use crate::parser::Parser;

pub fn run_ruby_program(text: String) -> String {
    let parser = Parser::new(Lexer::new(&text));
    let program = parser.parse();
    let mut module = wasm::module::Module::new();
    let mut ctx = add_core_items(&mut module);
    compiler::compile(&mut ctx, &program);
    let res = run::run_module(&mut ctx);
    res.to_pretty()
}
