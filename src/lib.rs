pub mod binary;
pub mod compiler;
pub mod corelib;
pub mod html;
pub mod lexeme;
pub mod lexer;
pub mod node;
pub mod parser;
pub mod run;
pub mod unitype;
pub mod wasm;
pub mod print_wat;

use wat_defs::module::Module;
pub use crate::compiler::CompileCtx;

use crate::corelib::add_core_items;
use crate::lexer::Lexer;
use crate::parser::Parser;

pub fn run_ruby_program(text: String) -> String {
    let parser = Parser::new(Lexer::new(&text));
    let program = parser.parse();
    let mut module = Module::new();
    let mut ctx = CompileCtx::new(&mut module);
    add_core_items(&mut ctx);
    compiler::compile(&mut ctx, &program);
    let res = run::run_module(&mut ctx);
    res.to_pretty()
}
