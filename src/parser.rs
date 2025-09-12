//! Ruby text -> AST parser

use crate::lexer::Lexer;
use crate::node::{Expr, Program, Statements};
use std::iter::Peekable;
use std::str::FromStr;
use std::vec;

use crate::lexeme::{Lexeme, LexemeKind as L, LexemeKind};

/// Strategy:
/// - Lex on-demand
/// - Output AST
/// - Recursive descent
/// - Use call stack as implicit stack of half-finished nodes
pub struct Parser<'text> {
    lexer: Lexer<'text>,
}

impl<'text> Parser<'text> {
    pub fn new(lexer: Lexer<'text>) -> Self {
        Self { lexer }
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
        Program { statements }
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

        Statements { body }
    }

    /// ```ebnf
    /// EXPR = keyword | integer_literal
    /// ```
    fn expr(&mut self) -> Option<Expr> {
        let lexeme = self.lexer.peek();
        match lexeme.kind {
            // Keywords
            L::IntegerLiteral { .. } => self.integer_literal(),
            L::True => Some(Expr::True),
            L::False => Some(Expr::False),
            L::Nil => Some(Expr::Nil),
            _ => None,
        }
    }

    /// Pre: `self.lexer.next().kind == LexemeKind::IntegerLiteral`
    fn integer_literal(&mut self) -> Option<Expr> {
        let lexeme = self.lexer.next();
        match lexeme.kind {
            L::IntegerLiteral { text } => Some(Expr::Integer(i64::from_str(&text).unwrap())),
            _ => unreachable!(),
        }
    }
}

// TODO `nil ;;;;;` is a valid ruby program.
#[cfg(test)]
mod tests {
    use crate::lexeme::LexemeKind::Eof;
    use super::*;

    fn lex_to_eof(text: &str) -> Vec<Lexeme> {
        let mut lexemes = vec![];
        let mut lexer = Lexer::new(text);
        loop {
            let lexeme = lexer.next();
            let eof= lexeme.kind == Eof;
            lexemes.push(lexeme);
            if eof {
                return lexemes;
            }
        }
    }

    // TODO -- There's gotta be a good way to test tree equality.
    //  I'm sure rust-analyzer has it figured out.
    //  I could just do everything with the textual output.
    #[test]
    fn integer_literal() {
        let text = "22";
        let parser = Parser::new(Lexer::new(text));
        let program = parser.parse();

        match program {
            Program { statements } => {
                assert_eq!(1, statements.body.len());
                assert_eq!(Expr::Integer(22), statements.body[0]);
            }
        }

        // let text = "-22_222";
        // let parser = Parser::new(Lexer::new(text));
        // let program = parser.parse();
        //
        // match program {
        //     Program { statements } => {
        //         assert_eq!(1, statements.body.len());
        //         assert_eq!(Expr::Integer(-22_222), statements.body[0]);
        //     }
        // }
    }
}
