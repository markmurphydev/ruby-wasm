//! Ruby text -> AST parser

use crate::lexer::Lexer;
use crate::node as N;
use std::iter::Peekable;
use std::str::FromStr;
use std::vec;

use crate::lexeme::{LexemeKind as LK, LexemeKind};

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

    pub fn parse(mut self) -> N::Program {
        self.program()
    }

    ///
    /// ```ebnf
    /// PROGRAM = STATEMENTS
    /// ```
    fn program(&mut self) -> N::Program {
        let statements = self.statements();
        N::Program { statements }
    }

    ///
    /// ```ebnf
    /// STATEMENTS = EXPR*
    /// ```
    fn statements(&mut self) -> N::Statements {
        let mut body = vec![];

        while let Some(expr) = self.expr() {
            body.push(expr);
        }

        N::Statements { body }
    }

    /// ```ebnf
    /// EXPR = keyword | integer_literal
    /// ```
    fn expr(&mut self) -> Option<N::Expr> {
        macro_rules! expect_simple_kw(
            ($lexeme_kind:path, $node:path) => ({
                self.expect(&[$lexeme_kind]);
                Some($node)
            })
        );

        let lexeme = self.lexer.peek();
        match lexeme.kind {
            LK::IntegerLiteral { .. } => self.integer_literal(),

            // Keywords
            LK::True => expect_simple_kw!(LK::True, N::Expr::True),
            LK::False => expect_simple_kw!(LK::False, N::Expr::False),
            LK::Nil => expect_simple_kw!(LK::Nil, N::Expr::Nil),

            // Control flow
            LK::If => Some(N::Expr::If(Box::new(self.if_expr()))),
            _ => None,
        }
    }

    /// Once we see "if", should be irrefutable.
    /// Pre: `self.lexer.next().kind == LexemeKind::If`
    fn if_expr(&mut self) -> N::If {
        // By default, we need to see an "end" lexeme.
        //  But if we see "elsif", then the nested `If`-expr will take care of it.
        let mut expects_end_lexeme = true;

        self.debug_expect(&[LK::If]);

        let predicate = self.expr().unwrap();
        self.expect(&[LK::Then]);

        let then_statements = self.statements();

        let subsequent: N::Subsequent = match self.lexer.peek().kind {
            LK::Else => {
                self.lexer.next(); // Consume "else"
                let else_statements = self.statements();
                N::Subsequent::Else(N::Else {
                    statements: else_statements,
                })
            }
            LK::Elsif => {
                expects_end_lexeme = false;
                let if_expr = self.if_expr();
                N::Subsequent::If(Box::new(if_expr))
            }
            _ => N::Subsequent::None
        };

        if expects_end_lexeme {
            self.expect(&[LK::End]);
        }

        N::If {
            predicate,
            statements: then_statements,
            subsequent,
        }
    }

    /// Pre: `self.lexer.next().kind == LexemeKind::IntegerLiteral`
    fn integer_literal(&mut self) -> Option<N::Expr> {
        let lexeme = self.lexer.next();
        match lexeme.kind {
            LK::IntegerLiteral { text } => Some(N::Expr::Integer(i64::from_str(&text).unwrap())),
            _ => unreachable!(),
        }
    }

    fn debug_expect(&mut self, expected: &[LexemeKind]) {
        let kind = self.lexer.next().kind;
        debug_assert!(expected.contains(&kind));
    }

    fn expect(&mut self, expected: &[LexemeKind]) {
        let kind = self.lexer.next().kind;
        assert!(expected.contains(&kind));
    }
}

// TODO `nil ;;;;;` is a valid ruby program.
#[cfg(test)]
mod tests {
    use crate::lexeme::Lexeme;
    use super::*;

    fn lex_to_eof(text: &str) -> Vec<Lexeme> {
        let mut lexemes = vec![];
        let mut lexer = Lexer::new(text);
        loop {
            let lexeme = lexer.next();
            let eof = lexeme.kind == LK::Eof;
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
            N::Program { statements } => {
                assert_eq!(1, statements.body.len());
                assert_eq!(N::Expr::Integer(22), statements.body[0]);
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
