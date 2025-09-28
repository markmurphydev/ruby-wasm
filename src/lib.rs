pub mod lexeme;
pub mod lexer;
pub mod node;
pub mod parser;
pub mod wasm;
pub mod compiler;
pub mod binary;
pub mod html;
pub mod runtime;
pub mod run;
pub mod unitype;

pub use wasm::instr_seq::InstrSeqBuilder;
pub use crate::wasm::wat;
pub use crate::compiler::CompileCtx;
use crate::lexer::Lexer;
use crate::parser::Parser;
pub use crate::wasm::function::FunctionBuilder;

pub fn run_text(text: String) -> String {
    let parser = Parser::new(Lexer::new(&text));
    let program = parser.parse();
    let module = compiler::compile(&program);
    run::run(module)
}
