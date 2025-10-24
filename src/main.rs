use clap::Parser as ParserTrait;
use clap::Subcommand;
use ruby_wasm::lexer::Lexer;
use ruby_wasm::parser::Parser;
use ruby_wasm::{CompileCtx, compiler};
use ruby_wasm::{binary, html, run};
use std::fs;
use wat_defs::module::Module;

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

    /// Compiles and runs the given program.
    Run {
        /// Text of program to compile
        text: String,
    },

    /// Compiles the given program, printing an `.html` file.
    /// The `.html` file has a button to compile and run the Wasm program.
    /// When the program is compiled, the `start` function will be called,
    /// and its output will be printed to the browser console.
    Html {
        /// Text of program to compile
        text: String,
    },

    WatToWasm {
        file: String,
    },

    /// Scratch area to test Rust language.
    /// TODO: Delete
    Scratch,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Lex { text } => {
            println!("{:?}", run::lex(&text));
        }

        Command::Parse { text } => {
            let parser = Parser::new(Lexer::new(&text));
            println!("{:?}", parser.parse());
        }

        Command::Compile { text } => {
            let parser = Parser::new(Lexer::new(&text));
            let program = parser.parse();
            let module = Module::new();
            let ctx = &mut CompileCtx::new(module);
            compiler::compile(ctx, &program);
            println!("{:?}", ctx.module);
        }

        Command::Wat { text } => {
            let wat = run::compile_ctx_to_wat(&run::text_to_compile_ctx(text));
            println!("{}", wat);
        }

        Command::Wasm { text } => {
            let ctx = run::text_to_compile_ctx(text);
            let bytes = binary::module_to_binary(&ctx.module);
            binary::print_bytes(&bytes);
        }

        Command::Run { text } => {
            println!("{}", run::run_text(text))
        }

        Command::Html { text } => {
            let ctx = run::text_to_compile_ctx(text);
            let bytes = binary::module_to_binary(&ctx.module);
            let html = html::make_html_wrapper(&bytes);
            println!("{}", html);
        }

        Command::WatToWasm { file } => {
            let wat = fs::read_to_string(file).unwrap();
            let bytes = wat::parse_str(wat).unwrap();
            binary::print_bytes(&bytes);
        }

        Command::Scratch => {
            let res = run::run_wat(fs::read_to_string("a.wat").unwrap());
            println!("{}", res);
        }
    }
}
