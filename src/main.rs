use clap::Parser as ParserTrait;
use clap::Subcommand;
use ruby_wasm::compiler::Compiler;
use ruby_wasm::lexeme::LexemeKind;
use ruby_wasm::lexer::Lexer;
use ruby_wasm::parser::Parser;
use ruby_wasm::wat::WatPrinter;
use ruby_wasm::{binary, html};

#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Lexes the given program, returning a list of its lexemes.
    Lex {
        /// Text to lex
        text: String,
    },

    /// Parses the given program, returning an abstract syntax tree
    Parse {
        /// Text to parse
        text: String,
    },

    /// Compiles the given program, returning a Wasm module
    Compile {
        /// Text of program to compile
        text: String,
    },

    /// Compiles the given program, printing a `.wat` text representation
    Wat {
        /// Text of program to compile
        text: String,
    },

    /// Compiles the given program, printing a `.wasm` binary representation
    Wasm {
        /// Text of program to compile
        text: String,
    },

    /// Compiles the given program, printing an `.html` file.
    /// The `.html` file has a button to compile and run the Wasm program.
    /// When the program is compiled, the `_start` function will be called,
    /// and its output will be printed to the browser console.
    Html {
        /// Text of program to compile
        text: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Lex { text } => {
            let mut lexer = Lexer::new(&text);
            loop {
                let lexeme = lexer.next();
                println!("{:?}", lexeme);
                if let LexemeKind::Eof = lexeme.kind {
                    return;
                }
            }
        }

        Command::Parse { text } => {
            let parser = Parser::new(Lexer::new(&text));
            println!("{:?}", parser.parse());
        }

        Command::Compile { text } => {
            let parser = Parser::new(Lexer::new(&text));
            let program = parser.parse();
            let wasm = Compiler.compile(program);
            println!("{:?}", wasm);
        }

        Command::Wat { text } => {
            let parser = Parser::new(Lexer::new(&text));
            let program = parser.parse();
            let wasm = Compiler.compile(program);
            let wat = WatPrinter::new().print_module(&wasm);
            println!("{}", wat);
        }

        Command::Wasm { text } => {
            let parser = Parser::new(Lexer::new(&text));
            let program = parser.parse();
            let module = Compiler.compile(program);
            let bytes = binary::module_to_binary(&module);
            binary::print_bytes(&bytes);
        }

        Command::Html { text } => {
            let parser = Parser::new(Lexer::new(&text));
            let program = parser.parse();
            let module = Compiler.compile(program);
            let bytes = binary::module_to_binary(&module);
            let html = html::make_html_wrapper(&bytes);
            println!("{}", html);
        }
    }
}
