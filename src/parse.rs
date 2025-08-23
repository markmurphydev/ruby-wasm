//! Ruby token -> wasm struct parser

use crate::token::Token;
use crate::wasm::{Expr, Function, FunctionIndex, Instruction, Module};
use itertools::PeekNth;
use std::vec;

const FALSE_VALUE: u64 = 0b0001;
const TRUE_VALUE: u64 = 0b0011;
const NIL_VALUE: u64 = 0b0111;

pub struct Parser {
    tokens: PeekNth<vec::IntoIter<Token>>,
    /// Instructions in the body of the function being constructed...
    instructions: Vec<Instruction>,
    functions: Vec<Function>,
    start: Option<FunctionIndex>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: itertools::peek_nth(tokens.into_iter()),
            instructions: vec![],
            functions: vec![],
            start: None,
        }
    }

    pub fn parse(mut self) -> Module {
        // On main file load, Ruby loads the Object class.
        // It adds top-level method definitions as private Object methods.
        // It adds
        // ^^^^ or something ^^^^
        // To start, let's just made an "outer" start function, and put everything in there
        // TODO -- Fix.

        while let Some(token) = self.tokens.peek() {
            use Token::*;
            match token {
                True | False | Nil => self.expression(),
                Newline | Semicolon => {
                    self.tokens.next();
                }
            }
        }

        let function = Function {
            name: None,
            body: Expr(self.instructions),
        };

        Module {
            functions: vec![function],
            start: Some(FunctionIndex::Index(0)),
        }
    }

    /// Consume an expression.
    fn expression(&mut self) {
        match self.tokens.next().unwrap() {
            first_token @ (Token::True | Token::False | Token::Nil) => {
                match self.tokens.peek() {
                    None => {
                        // An expression followed by end-of-file returns the value.
                        // TODO -- Fill out the wasm const instructions for each of these values.
                        match first_token {
                            Token::True => self.instructions.push(Instruction::TRUE),
                            Token::False => self.instructions.push(Instruction::FALSE),
                            Token::Nil => self.instructions.push(Instruction::NIL),
                            _ => panic!(),
                        }
                    }
                    Some(Token::Newline | Token::Semicolon) => {
                        self.tokens.next().unwrap();

                        // A value expression followed by a terminator is a no-op.
                        self.terminators()
                    }
                    Some(_) => {}
                }
            }
            _ => panic!("`expression`: unexpected token: "),
        }
    }

    /// Consume terminators \n and ;
    fn terminators(&mut self) {
        while let Some(Token::Semicolon | Token::Newline) = self.tokens.peek() {
            self.tokens.next();
        }
    }
}

// TODO `nil ;;;;;` is a valid ruby program.
#[cfg(test)]
mod tests {}
