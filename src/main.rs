use std::{env, fs};
use itertools::Itertools;
use ruby_wasm::parse::Parser;
use ruby_wasm::lexer::Lexer;
use ruby_wasm::wat::Printer;

fn main() {
    let mut args = env::args();
    assert_eq!(args.len(), 2);
    let file_name = args.nth(1).unwrap();

    let text = fs::read_to_string(file_name).unwrap();
    let tokens = Lexer::new(&text).tokenize();
    let module = Parser::new(tokens).parse();
    let wat = Printer::new().print_module(&module);
    fs::write("output.wat", wat).unwrap();
}
