//! Ruby text -> AST parser

use crate::lexeme::{Lexeme, LexemeKind as LK, LexemeKind};
use crate::lexer::Lexer;
use crate::node as N;
use std::str::FromStr;
use std::vec;

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

        self.skip_newlines();
        while let Some(expr) = self.expr() {
            body.push(expr);
            self.skip_newlines();
        }

        N::Statements { body }
    }

    /// ```ebnf
    /// EXPR = keyword | integer_literal
    /// ```
    fn expr(&mut self) -> Option<N::Expr> {
        self.expr_bp(0)
    }

    fn expr_bp(&mut self, min_bp: u8) -> Option<N::Expr> {
        /// Expect the given keyword's lexem kind, then return its node
        macro_rules! expect_simple_kw(
            ($lexeme_kind:path, $node_expr_variant:path) => ({
                self.expect(&[$lexeme_kind]);
                Some($node_expr_variant)
            })
        );

        /// Wrap the given `expr` with `$node_expr_variant(box expr)`
        macro_rules! box_expr_variant(
            ($expr:expr, $node_expr_variant:path) => ((
                Some($node_expr_variant(Box::new($expr)))
            ))
        );

        let lexeme = self.lexer.peek();
        let lhs = match lexeme.kind {
            // Literals
            LK::IntegerLiteral { .. } => self.integer_literal(),
            LK::SingleQuoteStringLiteral { .. } => self.single_quote_string_literal(),
            LK::BracketLeftRight { .. } => {
                self.lexer.next();
                Some(N::Expr::Array(Box::new(N::Array { vals: vec![] })))
            }
            LK::BracketLeft { .. } => box_expr_variant!(self.array_literal(), N::Expr::Array),

            // Keywords
            LK::True => expect_simple_kw!(LK::True, N::Expr::True),
            LK::False => expect_simple_kw!(LK::False, N::Expr::False),
            LK::Nil => expect_simple_kw!(LK::Nil, N::Expr::Nil),

            LK::GlobalVariable { .. } => {
                box_expr_variant!(self.global_variable(), N::Expr::GlobalVariableRead)
            }
            LK::Constant { .. } => Some(self.constant()),
            LK::Identifier { .. } => Some(self.parse_ident(min_bp)),

            // Control flow
            LK::If => box_expr_variant!(self.if_expr(), N::Expr::If),
            LK::While => box_expr_variant!(self.while_expr(), N::Expr::While),
            LK::Until => box_expr_variant!(self.until_expr(), N::Expr::Until),

            LK::Minus => self.unary_minus(),

            LK::Def => box_expr_variant!(self.def_expr(), N::Expr::Def),

            _ => None,
        };

        let Some(mut lhs) = lhs else { return None };

        // Parse infix operators
        loop {
            let op = match self.lexer.peek() {
                op if op.is_operator() => op,
                _ => break,
            };
            let (l_bp, r_bp) = op.binding_power();
            if l_bp < min_bp {
                break;
            }

            self.lexer.next();
            lhs = match op.kind {
                LK::Dot => {
                    let name = match self.lexer.next().kind {
                        LK::Identifier { text: name } => name,
                        LK::BracketLeftRight => "[]".to_string(),
                        _ => panic!("Expected identifier or `[]`."),
                    };

                    self.expect(&[LK::LeftParen]);
                    let args = self.args(LK::RightParen);

                    N::Expr::Call(Box::new(N::Call {
                        receiver: Some(lhs),
                        name,
                        args,
                    }))
                }
                LK::Equal => match lhs {
                    N::Expr::GlobalVariableRead(glob) => {
                        let rhs = self.expr_bp(r_bp).unwrap();
                        let N::GlobalVariableRead { name } = *glob;
                        N::Expr::GlobalVariableWrite(Box::new(N::GlobalVariableWrite {
                            name,
                            expr: rhs,
                        }))
                    }
                    N::Expr::Call(call) if call.name == "[]" => {
                        let rhs = self.expr_bp(r_bp).unwrap();
                        N::Expr::Call(Box::new(N::Call {
                            receiver: call.receiver,
                            name: "[]=".to_string(),
                            args: [call.args, vec![rhs]].concat(),
                        }))
                    }
                    _ => todo!("Unknown assignment lhs: {:?}", lhs),
                },
                LK::AmpersandAmpersand => {
                    let rhs = self.expr_bp(r_bp).unwrap();
                    N::Expr::And(Box::new(N::And { lhs, rhs }))
                }
                LK::PipePipe => {
                    let rhs = self.expr_bp(r_bp).unwrap();

                    N::Expr::Or(Box::new(N::Or { lhs, rhs }))
                }
                op @ (LK::EqualEqual
                | LK::Greater
                | LK::Less
                | LK::Minus
                | LK::Plus
                | LK::Slash
                | LK::Star) => {
                    let name = match op {
                        LK::EqualEqual => "==".to_string(),
                        LK::Greater => ">".to_string(),
                        LK::Less => "<".to_string(),
                        LK::Minus => "-".to_string(),
                        LK::Plus => "+".to_string(),
                        LK::Slash => "/".to_string(),
                        LK::Star => "*".to_string(),
                        _ => unreachable!(),
                    };
                    let rhs = self.expr_bp(r_bp).unwrap();

                    N::Expr::Call(Box::new(N::Call {
                        receiver: Some(lhs),
                        name,
                        args: vec![rhs],
                    }))
                }
                LK::BracketLeft => {
                    let args = self.args(LK::BracketRight);
                    assert_eq!(1, args.len());

                    N::Expr::Call(Box::new(N::Call {
                        receiver: Some(lhs),
                        name: "[]".to_string(),
                        args,
                    }))
                }
                other => unreachable!("Lexeme kind {:?} is not an operator.", other),
            };
        }
        Some(lhs)
    }

    fn parse_ident(&mut self, min_bp: u8) -> N::Expr {
        let LK::Identifier { text: name } = self.lexer.next().kind else {
            unreachable!()
        };
        match self.lexer.peek().kind {
            LK::LeftParen => {
                self.lexer.next();
                let args = self.args(LK::RightParen);
                N::Expr::Call(Box::new(N::Call {
                    receiver: None,
                    name,
                    args,
                }))
            }
            LK::Equal => {
                let (l_bp, r_bp) = LK::Equal.binding_power();
                if l_bp < min_bp {
                    return N::Expr::LocalVariableRead(Box::new(N::LocalVariableRead { name }));
                }
                let rhs = self.expr_bp(r_bp).unwrap();
                N::Expr::LocalVariableWrite(Box::new(N::LocalVariableWrite { name, val: rhs }))
            }
            _ => N::Expr::LocalVariableRead(Box::new(N::LocalVariableRead { name })),
        }
    }

    fn def_expr(&mut self) -> N::Def {
        self.lexer.next();
        let LK::Identifier { text: name } = self.lexer.next().kind else {
            panic!("Expected method name after `def`.")
        };
        self.expect(&[LK::LeftParen]);
        let params = self.params();
        self.skip_newlines();
        let body = self.statements();
        self.skip_newlines();
        self.expect(&[LK::End]);

        N::Def { name, params, body }
    }

    fn params(&mut self) -> Vec<N::RequiredParam> {
        let mut params = vec![];
        loop {
            match self.lexer.peek().kind {
                LK::Identifier { text } => {
                    self.lexer.next();
                    params.push(N::RequiredParam { name: text });
                    match self.lexer.next().kind {
                        LK::Comma => continue,
                        LK::RightParen => break,
                        _ => panic!("Expected `,` or `)`."),
                    }
                }
                LK::RightParen => {
                    self.lexer.next();
                    break;
                }
                _ => panic!("Expected identifier or `)`."),
            }
        }
        params
    }

    fn unary_minus(&mut self) -> Option<N::Expr> {
        self.expect(&[LK::Minus]);
        let lhs = self.expr_bp(Lexeme::UNARY_MINUS_BINDING_POWER).unwrap();
        Some(N::Expr::Call(Box::new(N::Call {
            receiver: Some(lhs),
            name: "-@".to_string(),
            args: vec![],
        })))
    }

    /// Parse args until terminator.
    /// Pre: Opening `(` has been consumed
    fn args(&mut self, terminator: LexemeKind) -> Vec<N::Expr> {
        let mut args = vec![];
        while let Some(arg) = self.expr() {
            args.push(arg);

            match self.lexer.peek().kind {
                LK::Comma => {
                    self.lexer.next();
                }
                lk if lk == terminator => break,
                _ => panic!(),
            }
        }

        self.lexer.next();
        args
    }

    fn array_literal(&mut self) -> N::Array {
        self.debug_expect(&[LK::BracketLeft]);

        let mut vals = vec![];
        loop {
            vals.push(self.expr().unwrap());
            match self.lexer.next().kind {
                LK::Comma => {}
                LK::BracketRight => break,
                _ => panic!("Expected `,` or `]`"),
            }
        }
        N::Array { vals }
    }

    /// Once we see "if", should be irrefutable.
    /// Pre: `self.lexer.next().kind == LexemeKind::If`
    fn if_expr(&mut self) -> N::If {
        // By default, we need to see an "end" lexeme.
        //  But if we see "elsif", then the nested `If`-expr will take care of it.
        let mut expects_end_lexeme = true;

        self.debug_expect(&[LK::If]);
        let predicate = self.expr().unwrap();
        self.consume_if_found(LK::Then);
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
                N::Subsequent::Elsif(Box::new(if_expr))
            }
            _ => N::Subsequent::None,
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

    /// Parse global variable into a `GlobalRead`
    /// Pre: `self.lexer.next().kind == LexemeKind::GlobalVariable`
    fn global_variable(&mut self) -> N::GlobalVariableRead {
        let global = self.lexer.next();
        let Lexeme {
            kind: LK::GlobalVariable { text },
            ..
        } = global
        else {
            unreachable!("Not a `GlobalVariable`: {:?}", global)
        };

        let name = text.chars().skip(1).collect();
        N::GlobalVariableRead { name }
    }

    /// Parse a constant into either `ConstantWrite` or `ConstantRead`
    /// Pre: `self.lexer.next().kind == LexemeKind::Constant`
    fn constant(&mut self) -> N::Expr {
        let constant = self.lexer.next();
        let Lexeme {
            kind: LK::Constant { text },
            ..
        } = constant
        else {
            unreachable!("Not a `Constant`: {:?}", constant)
        };

        match self.lexer.peek().kind {
            LK::Equal => {
                self.lexer.next();
                let rhs = self.expr().expect("Assignment with no RHS");
                N::Expr::ConstantWrite(Box::new(N::ConstantWrite {
                    name: text,
                    expr: rhs,
                }))
            }
            _ => N::Expr::ConstantRead(Box::new(N::ConstantRead { name: text })),
        }
    }

    /// Once we see "while", should be irrefutable.
    /// Pre: `self.lexer.next().kind == LexemeKind::While`
    fn while_expr(&mut self) -> N::While {
        self.debug_expect(&[LK::While]);
        let predicate = self.expr().unwrap();
        self.consume_if_found(LK::Do);
        let statements = self.statements();
        self.expect(&[LK::End]);

        N::While {
            predicate,
            statements,
        }
    }

    /// Once we see "until", should be irrefutable.
    /// Pre: `self.lexer.next().kind == LexemeKind::Until`
    fn until_expr(&mut self) -> N::Until {
        self.debug_expect(&[LK::Until]);
        let predicate = self.expr().unwrap();
        self.consume_if_found(LK::Do);
        let statements = self.statements();
        self.expect(&[LK::End]);

        N::Until {
            predicate,
            statements,
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

    /// Pre: `self.lexer.next().kind == LexemeKind::SingleQuoteStringLiteral`
    fn single_quote_string_literal(&mut self) -> Option<N::Expr> {
        let lexeme = self.lexer.next();
        match lexeme.kind {
            LK::SingleQuoteStringLiteral { text } => {
                let inner = text[1..text.len() - 1].to_string();
                Some(N::Expr::SingleQuoteString(inner))
            }
            _ => unreachable!(),
        }
    }

    /// Peek the next token. If it's of kind `expected`, consume it.
    fn consume_if_found(&mut self, expected: LK) -> Option<Lexeme> {
        match self.lexer.peek() {
            Lexeme { kind, .. } if kind == expected => Some(self.lexer.next()),
            _ => None,
        }
    }

    /// expected: Set of acceptable lexeme kinds
    fn debug_expect(&mut self, expected: &[LexemeKind]) -> Lexeme {
        let lexeme = self.lexer.next();
        debug_assert!(expected.contains(&lexeme.kind));
        lexeme
    }

    /// expected: Set of acceptable lexeme kinds
    fn expect(&mut self, expected: &[LexemeKind]) -> Lexeme {
        let lexeme = self.lexer.next();
        assert!(expected.contains(&lexeme.kind));
        lexeme
    }

    fn skip_newlines(&mut self) {
        while let LK::Newline = self.lexer.peek().kind {
            self.lexer.next();
        }
    }
}

// TODO `nil ;;;;;` is a valid ruby program.
#[cfg(test)]
mod tests {
    use super::*;
    use expect_test::expect;
    use serde_lexpr::print;
    use std::process::Command;

    /// Parse the given text
    /// and return pretty-printed sexpr
    /// This only works so long as the lexer can be driven independently of the parser.
    fn parse_to_sexpr(text: &str) -> String {
        let parser = Parser::new(Lexer::new(text));
        let program = parser.parse();

        // TODO: formatting as elisp, cause we're using emacs for pretty-printing...
        let sexpr = serde_lexpr::to_string_custom(&program, print::Options::elisp()).unwrap();
        format_sexpr(&sexpr)
    }

    /// Pretty print given sexpr
    /// TODO -- Remove the emacs dependency...
    fn format_sexpr(sexpr: &str) -> String {
        let pp_command = format!(
            "(pp (car (read-from-string \"{}\"))))",
            sexpr.replace(r#"""#, r#"\""#)
        );
        let output = Command::new("emacs")
            .args(["--batch", "--eval", &pp_command])
            .output()
            .unwrap();
        String::from_utf8(output.stdout).unwrap()
    }

    // TODO -- There's gotta be a good way to test tree equality.
    //  I'm sure rust-analyzer has it figured out.
    //  I could just do everything with the textual output.
    #[test]
    fn integer_literal() {
        let text = "22";
        let parser = Parser::new(Lexer::new(text));
        let program = parser.parse();

        let N::Program { statements } = program;
        assert_eq!(1, statements.body.len());
        assert_eq!(N::Expr::Integer(22), statements.body[0]);
    }

    #[test]
    fn single_quote_string_literal() {
        let text = "'22'";
        let parser = Parser::new(Lexer::new(text));
        let program = parser.parse();

        match program {
            N::Program { statements } => {
                assert_eq!(1, statements.body.len());
                // Strips quotes
                assert_eq!(
                    N::Expr::SingleQuoteString("22".to_string()),
                    statements.body[0]
                );
            }
        }
    }

    #[test]
    fn integer() {
        let text = "'22'";
        let parser = Parser::new(Lexer::new(text));
        let program = parser.parse();

        let N::Program { statements } = program;
        assert_eq!(1, statements.body.len());
        // Strips quotes
        assert_eq!(
            N::Expr::SingleQuoteString("22".to_string()),
            statements.body[0]
        );
    }

    #[test]
    fn single_addition() {
        let text = "1 + 2";
        let expected = expect![[r#"
            ((statements
              (body
               (Call (receiver (Integer . 1)) (name . "+") (args (Integer . 2))))))
        "#]];
        let actual = parse_to_sexpr(text);
        expected.assert_eq(&actual);
    }

    #[test]
    fn multiple_addition() {
        let text = "1 + 2 + 3";
        let expected = expect![[r#"
            ((statements
              (body
               (Call
                (receiver
                 (Call (receiver (Integer . 1)) (name . "+") (args (Integer . 2))))
                (name . "+") (args (Integer . 3))))))
        "#]];
        let actual = parse_to_sexpr(text);
        expected.assert_eq(&actual);
    }

    #[test]
    fn add_sub_mul_div() {
        let text = "1 + 2 - 3 * 4 / 5";
        let expected = expect![[r#"
            ((statements
              (body
               (Call
                (receiver
                 (Call (receiver (Integer . 1)) (name . "+") (args (Integer . 2))))
                (name . "-")
                (args
                 (Call
                  (receiver
                   (Call (receiver (Integer . 3)) (name . "*")
            	     (args (Integer . 4))))
                  (name . "/") (args (Integer . 5))))))))
        "#]];
        let actual = parse_to_sexpr(text);
        expected.assert_eq(&actual);
    }

    #[test]
    fn matklad_example() {
        // Example from https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html
        let text = "1 + 2 * 3 * 4 + 5";
        // (+ (+ 1
        //       (* (* 2 3)
        //          4)
        //    5)
        let expected = expect![[r#"
            ((statements
              (body
               (Call
                (receiver
                 (Call (receiver (Integer . 1)) (name . "+")
            	   (args
            	    (Call
            	     (receiver
            	      (Call (receiver (Integer . 2)) (name . "*")
            		    (args (Integer . 3))))
            	     (name . "*") (args (Integer . 4))))))
                (name . "+") (args (Integer . 5))))))
        "#]];
        let actual = parse_to_sexpr(text);
        expected.assert_eq(&actual);
    }

    #[test]
    fn method_call() {
        let text = "$foo.bar()";
        let expected = expect![[r#"
            ((statements
              (body
               (Call (receiver (GlobalVariableRead (name . "foo"))) (name . "bar")
            	 (args)))))
        "#]];
        let actual = parse_to_sexpr(text);
        expected.assert_eq(&actual);
    }

    #[test]
    fn method_call_multiple_args() {
        let text = "$foo.bar(1, 2, 3)";
        let expected = expect![[r#"
            ((statements
              (body
               (Call (receiver (GlobalVariableRead (name . "foo"))) (name . "bar")
            	 (args (Integer . 1) (Integer . 2) (Integer . 3))))))
        "#]];
        let actual = parse_to_sexpr(text);
        expected.assert_eq(&actual);
    }

    #[test]
    fn method_call_in_operators() {
        let text = "1 + $foo.bar() * 2 + 3";
        let expected = expect![[r#"
            ((statements
              (body
               (Call
                (receiver
                 (Call (receiver (Integer . 1)) (name . "+")
            	   (args
            	    (Call
            	     (receiver
            	      (Call (receiver (GlobalVariableRead (name . "foo")))
            		    (name . "bar") (args)))
            	     (name . "*") (args (Integer . 2))))))
                (name . "+") (args (Integer . 3))))))
        "#]];
        let actual = parse_to_sexpr(text);
        expected.assert_eq(&actual);
    }

    #[test]
    fn unary_minus() {
        let text = "-1";
        let expected = expect![[r#"
            ((statements
              (body (Call (receiver (Integer . 1)) (name . "-@") (args)))))
        "#]];
        let actual = parse_to_sexpr(text);
        expected.assert_eq(&actual);
    }

    #[test]
    fn unary_minus_multi_digit() {
        let text = "-9999";
        let expected = expect![[r#"
            ((statements
              (body (Call (receiver (Integer . 9999)) (name . "-@") (args)))))
        "#]];
        let actual = parse_to_sexpr(text);
        expected.assert_eq(&actual);
    }

    #[test]
    fn logical_ops() {
        let text = "false && true || false";
        let expected = expect![[r#"
            ((statements
              (body (Or (lhs And (lhs . False) (rhs . True)) (rhs . False)))))
        "#]];
        let actual = parse_to_sexpr(text);
        expected.assert_eq(&actual);
    }

    #[test]
    fn gt_lt() {
        let text = "1 < 2 && 3 < 4";
        let expected = expect![[r#"
            ((statements
              (body
               (And
                (lhs Call (receiver (Integer . 1)) (name . "<")
            	 (args (Integer . 2)))
                (rhs Call (receiver (Integer . 3)) (name . "<")
            	 (args (Integer . 4)))))))
        "#]];
        let actual = parse_to_sexpr(text);
        expected.assert_eq(&actual);
    }

    #[test]
    fn call_brackets() {
        let text = "true.[](0)";
        let expected = expect![[r#"
            ((statements
              (body (Call (receiver True) (name . "[]") (args (Integer . 0))))))
        "#]];
        let actual = parse_to_sexpr(text);
        expected.assert_eq(&actual);
    }

    #[test]
    fn parse_eq_eq() {
        let text = "22 == 44";
        let expected = expect![[r#"
            ((statements
              (body
               (Call (receiver (Integer . 22)) (name . "==") (args (Integer . 44))))))
        "#]];
        let actual = parse_to_sexpr(text);
        expected.assert_eq(&actual);
    }

    #[test]
    fn parse_complicated_equal_equal() {
        let text = "$a[44] + 2 * $b == $c[$d]";
        let expected = expect![[r#"
            ((statements
              (body
               (Call
                (receiver
                 (Call
                  (receiver
                   (Call (receiver (GlobalVariableRead (name . "a")))
            	     (name . "[]") (args (Integer . 44))))
                  (name . "+")
                  (args
                   (Call (receiver (Integer . 2)) (name . "*")
            	     (args (GlobalVariableRead (name . "b")))))))
                (name . "==")
                (args
                 (Call (receiver (GlobalVariableRead (name . "c"))) (name . "[]")
            	   (args (GlobalVariableRead (name . "d")))))))))
        "#]];
        let actual = parse_to_sexpr(text);
        expected.assert_eq(&actual);
    }

    #[test]
    fn parse_def() {
        let text = "
            def x(n)
                n
            end
        ";
        let expected = expect![[r#"
            ((statements
              (body
               (Def (name . "x") (params ((name . "n")))
            	(body (body (LocalVariableRead (name . "n"))))))))
        "#]];
        let actual = parse_to_sexpr(text);
        expected.assert_eq(&actual);
    }

    #[test]
    fn def_no_args() {
        let text = "
            def x()
                22
            end
        ";
        let expected = expect![[r#"
            ((statements
              (body (Def (name . "x") (params) (body (body (Integer . 22)))))))
        "#]];
        let actual = parse_to_sexpr(text);
        expected.assert_eq(&actual);
    }

    #[test]
    fn def_multiple_args() {
        let text = "
            def x(a, b)
                a + b
            end
        ";
        let expected = expect![[r#"
            ((statements
              (body
               (Def (name . "x") (params ((name . "a")) ((name . "b")))
            	(body
            	 (body
            	  (Call (receiver (LocalVariableRead (name . "a")))
            		(name . "+") (args (LocalVariableRead (name . "b"))))))))))
        "#]];
        let actual = parse_to_sexpr(text);
        expected.assert_eq(&actual);
    }

    #[test]
    fn method_call_no_receiver() {
        let text = "x()";
        let expected = expect![[r#"
            ((statements (body (Call (receiver) (name . "x") (args)))))
        "#]];
        let actual = parse_to_sexpr(text);
        expected.assert_eq(&actual);
    }

    #[test]
    fn def_then_call() {
        let text = "
            def x(n)
                n
            end
            x(22)
        ";
        let expected = expect![[r#"
            ((statements
              (body
               (Def (name . "x") (params ((name . "n")))
            	(body (body (LocalVariableRead (name . "n")))))
               (Call (receiver) (name . "x") (args (Integer . 22))))))
        "#]];
        let actual = parse_to_sexpr(text);
        expected.assert_eq(&actual);
    }

    mod globals {
        use crate::parser::tests::parse_to_sexpr;
        use expect_test::expect;

        #[test]
        fn global_read() {
            let text = "$asdf";
            let expected = expect![[r#"
                ((statements (body (GlobalVariableRead (name . "asdf")))))
            "#]];
            let actual = parse_to_sexpr(text);
            expected.assert_eq(&actual);
        }

        #[test]
        fn global_write() {
            let text = "$asdf = 22";
            let expected = expect![[r#"
                ((statements
                  (body (GlobalVariableWrite (name . "asdf") (expr Integer . 22)))))
            "#]];
            let actual = parse_to_sexpr(text);
            expected.assert_eq(&actual);
        }

        #[test]
        fn global_write_then_read() {
            let text = "$asdf = 22
            $asdf";
            let expected = expect![[r#"
                ((statements
                  (body (GlobalVariableWrite (name . "asdf") (expr Integer . 22))
                	(GlobalVariableRead (name . "asdf")))))
            "#]];
            let actual = parse_to_sexpr(text);
            expected.assert_eq(&actual);
        }
    }

    mod constants {
        use crate::parser::tests::parse_to_sexpr;
        use expect_test::expect;

        #[test]
        fn object() {
            let text = "Object";
            let expected = expect![[r#"
                ((statements (body (ConstantRead (name . "Object")))))
            "#]];
            let actual = parse_to_sexpr(text);
            expected.assert_eq(&actual);
        }
    }

    mod arrays {
        use crate::parser::tests::parse_to_sexpr;

        use expect_test::expect;

        #[test]
        fn empty_arr() {
            let text = "[]";
            let expected = expect![[r#"
                ((statements (body (Array (vals)))))
            "#]];
            let actual = parse_to_sexpr(text);
            expected.assert_eq(&actual);
        }

        #[test]
        fn non_empty_arr() {
            let text = "[1, 2, 3]";
            let expected = expect![[r#"
                ((statements
                  (body (Array (vals (Integer . 1) (Integer . 2) (Integer . 3))))))
            "#]];
            let actual = parse_to_sexpr(text);
            expected.assert_eq(&actual);
        }

        #[test]
        fn array_indexing() {
            let text = "[1, 2, 3][1]";
            let expected = expect![[r#"
                ((statements
                  (body
                   (Call
                    (receiver (Array (vals (Integer . 1) (Integer . 2) (Integer . 3))))
                    (name . "[]") (args (Integer . 1))))))
            "#]];
            let actual = parse_to_sexpr(text);
            expected.assert_eq(&actual);
        }

        #[test]
        fn array_index_assignment() {
            let text = "$x = [1, 2, 3]
                $x[1] = 0
                $x";
            let expected = expect![[r#"
                ((statements
                  (body
                   (GlobalVariableWrite (name . "x")
                			(expr Array
                			      (vals (Integer . 1) (Integer . 2)
                				    (Integer . 3))))
                   (Call (receiver (GlobalVariableRead (name . "x"))) (name . "[]=")
                	 (args (Integer . 1) (Integer . 0)))
                   (GlobalVariableRead (name . "x")))))
            "#]];
            let actual = parse_to_sexpr(text);
            expected.assert_eq(&actual);
        }
    }
}
