//! Ruby text -> AST parser

use std::iter::Peekable;
use std::vec;
use crate::lexer::Lexer;
use crate::node::{Expr, Program, Statements};

use crate::lexeme::LexemeKind as L;

/// Strategy:
/// - Lex on-demand
/// - Output AST
/// - Recursive descent
/// - Use call stack as implicit stack of half-finished nodes
pub struct Parser<'text> {
    lexer: Peekable<Lexer<'text>>
}

impl <'text> Parser<'text> {
    pub fn new(lexer: Lexer<'text>) -> Self {
        Self {
            lexer: lexer.peekable()
        }
    }

    pub fn parse(mut self) -> Program {
        self.program()
    }

    ///
    /// ```ebnf
    /// PROGRAM = STATEMENTS
    /// ```
    fn program(&mut self) -> Program {
        let statements = self.statements();
        Program {
            statements
        }
    }

    ///
    /// ```ebnf
    /// STATEMENTS = EXPR*
    /// ```
    fn statements(&mut self) -> Statements {
        let mut body = vec![];

        while let Some(expr) = self.expr() {
            body.push(expr);
        }

        Statements {
            body,
        }
    }

    /// ```ebnf
    /// EXPR = keyword
    /// ```
    fn expr(&mut self) -> Option<Expr> {
        let expr = match self.lexer.peek() {
            // TODO -- This is erroneously made possible by `impl <'text> Iterator for Lexer<'text>`
            None => panic!("Lexer finished iteration before EOF"),
            Some(lexeme) => match lexeme.kind {
                L::True => Some(Expr::True),
                L::False => Some(Expr::False),
                L::Nil => Some(Expr::Nil),
                _ => None
            }
        };

        if let Some(_) = expr {
            self.lexer.next();
        }

        expr
    }

    // pub fn parse(mut self) -> Program {
    //     // On main file load, Ruby loads the Object class.
    //     // It adds top-level method definitions as private Object methods.
    //     // It adds
    //     // ^^^^ or something ^^^^
    //     // To start, let's just made an "outer" start function, and put everything in there
    //     // TODO -- Fix.
    //
    //     while let Some(token) = self.tokens.peek() {
    //         use LexemeKind::*;
    //         match token {
    //             True | False | Nil => self.expression(),
    //             Newline | Semicolon => {
    //                 self.tokens.next();
    //             }
    //         }
    //     }
    //
    //     let function = Function {
    //         name: None,
    //         body: Expr(self.instructions),
    //     };
    //
    //     Module {
    //         functions: vec![function],
    //         start: Some(FunctionIndex::Index(0)),
    //     }
    // }

    // /// Consume an expression.
    // fn expression(&mut self) {
    //     match self.tokens.next().unwrap() {
    //         first_token @ (LexemeKind::True | LexemeKind::False | LexemeKind::Nil) => {
    //             match self.tokens.peek() {
    //                 None => {
    //                     // An expression followed by end-of-file returns the value.
    //                     // TODO -- Fill out the wasm const instructions for each of these values.
    //                     match first_token {
    //                         LexemeKind::True => self.instructions.push(Instruction::TRUE),
    //                         LexemeKind::False => self.instructions.push(Instruction::FALSE),
    //                         LexemeKind::Nil => self.instructions.push(Instruction::NIL),
    //                         _ => panic!(),
    //                     }
    //                 }
    //                 Some(LexemeKind::Newline | LexemeKind::Semicolon) => {
    //                     self.tokens.next().unwrap();
    //
    //                     // A value expression followed by a terminator is a no-op.
    //                     self.terminators()
    //                 }
    //                 Some(_) => {}
    //             }
    //         }
    //         _ => panic!("`expression`: unexpected token: "),
    //     }
    // }
    //
    // /// Consume terminators \n and ;
    // fn terminators(&mut self) {
    //     while let Some(LexemeKind::Semicolon | LexemeKind::Newline) = self.tokens.peek() {
    //         self.tokens.next();
    //     }
    // }
}

// TODO `nil ;;;;;` is a valid ruby program.
#[cfg(test)]
mod tests {}
