use std::{env, fs};
use clap::{Parser, Subcommand};
use ruby_wasm::lexer::Lexer;
use ruby_wasm::lexeme::{Lexeme, LexemeKind};

#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command
}

#[derive(Subcommand)]
enum Command {
    /// Lexes the given program, returning a list of its tokens.
    Lex {
        /// Text to lex
        text: String,
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Lex { text } => {
            loop {
                let lexeme = Lexer::new(&text).lex();
                println!("{:?}", lexeme);
                if let LexemeKind::Eof = lexeme.kind {
                    return;
                }
            }
        }
    }
}
