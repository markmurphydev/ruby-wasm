use crate::lexeme::{Col, Lexeme, LexemeKind, Line};
use itertools::{Itertools, PeekNth};
use std::str::Chars;

use LexemeKind::*;

/// Lexes Ruby program input.
/// Attempts to produce identical output to the Prism lexer.
/// NOTE: If we find ourselves adding state to this struct, consider whether
/// it would be better added as parser-driven input to the lex method
pub struct Lexer<'text> {
    /// I think `PeekNth` is the most convenient way to interact with a
    /// sequence of characters in Rust, so I'm using it here.
    /// TODO: Do we need multi-character peek?
    chars: PeekNth<Chars<'text>>,
    line: Line,
    col: Col,
}

impl<'text> Lexer<'text> {
    //! Invariant:
    //!     Coords are not updated (outside skips) until directly before returning the lexeme.

    pub fn new(text: &'text str) -> Self {
        Self {
            chars: itertools::peek_nth(text.chars()),
            line: 1,
            col: 0,
        }
    }

    /// Consume and return a single lexeme
    pub fn lex(&mut self) -> Lexeme {
        // Strategy: For punctuation, simple matching.

        // Prism doesn't include whitespace in token width.
        self.skip_whitespace();

        match self.chars.next() {
            // Prism EOF is 0-width.
            None => Lexeme::new(Eof, self.line, self.col, self.line, self.col),
            Some(c) => match c {
                '\n' => self.newline(),
                
                // Character literal

                // '?' can be the start of a character literal, or a ternary operator
                '?' => match self.chars.peek() {
                    None => self.simple_lexeme(Question, 1),
                    Some(c) if c.is_whitespace() => self.simple_lexeme(Question, 1),
                    _ => self.character_literal()
                }

                // Punctuation
                '&' => match self.chars.peek() {
                    Some('&') => {
                        self.chars.next();
                        match self.chars.peek() {
                            Some('=') => {
                                self.chars.next();
                                self.simple_lexeme(AmpersandAmpersandEqual, 3)
                            }
                            _ => self.simple_lexeme(AmpersandAmpersand, 2),
                        }
                    }
                    Some('=') => {
                        self.chars.next();
                        self.simple_lexeme(AmpersandEqual, 2)
                    }
                    Some('.') => {
                        self.chars.next();
                        self.simple_lexeme(AmpersandPeriod, 2)
                    }
                    _ => self.simple_lexeme(Ampersand, 1),
                },
                '`' => self.simple_lexeme(Backtick, 1),
                '!' => match self.chars.peek() {
                    Some('=') => {
                        self.chars.next();
                        self.simple_lexeme(BangEqual, 2)
                    }
                    Some('~') => {
                        self.chars.next();
                        self.simple_lexeme(BangTilde, 2)
                    }
                    _ => self.simple_lexeme(Bang, 1),
                },
                '{' => self.simple_lexeme(BraceLeft, 1),
                '}' => self.simple_lexeme(BraceRight, 1),
                '[' => match self.chars.peek() {
                    Some(']') => {
                        self.chars.next();
                        match self.chars.peek() {
                            Some('=') => {
                                self.chars.next();
                                self.simple_lexeme(BracketLeftRightEqual, 3)
                            }
                            _ => self.simple_lexeme(BracketLeftRight, 2),
                        }
                    }
                    _ => self.simple_lexeme(BracketLeft, 1),
                },
                ']' => self.simple_lexeme(BracketRight, 1),
                '^' => match self.chars.peek() {
                    Some('=') => {
                        self.chars.next();
                        self.simple_lexeme(CaretEqual, 2)
                    }
                    _ => self.simple_lexeme(Caret, 1),
                },
                ':' => match self.chars.peek() {
                    Some(':') => {
                        self.chars.next();
                        self.simple_lexeme(ColonColon, 2)
                    }
                    _ => self.simple_lexeme(Colon, 1),
                },
                ',' => self.simple_lexeme(Comma, 1),
                '.' => match self.chars.peek() {
                    Some('.') => {
                        self.chars.next();
                        match self.chars.peek() {
                            Some('.') => {
                                self.chars.next();
                                self.simple_lexeme(DotDotDot, 3)
                            }
                            _ => self.simple_lexeme(DotDot, 2),
                        }
                    }
                    _ => self.simple_lexeme(Dot, 1),
                },
                '=' => match self.chars.peek() {
                    Some('=') => {
                        self.chars.next();
                        match self.chars.peek() {
                            Some('=') => {
                                self.chars.next();
                                self.simple_lexeme(EqualEqualEqual, 3)
                            }
                            _ => self.simple_lexeme(EqualEqual, 2),
                        }
                    }
                    Some('>') => self.simple_lexeme(EqualGreater, 2),
                    Some('~') => self.simple_lexeme(EqualTilde, 2),
                    _ => self.simple_lexeme(Equal, 1),
                },
                '>' => match self.chars.peek() {
                    Some('>') => {
                        self.chars.next();
                        match self.chars.peek() {
                            Some('=') => {
                                self.chars.next();
                                self.simple_lexeme(GreaterGreaterEqual, 3)
                            }
                            _ => self.simple_lexeme(GreaterGreater, 2),
                        }
                    }
                    Some('=') => self.simple_lexeme(GreaterEqual, 2),
                    _ => self.simple_lexeme(Greater, 1),
                },
                '<' => match self.chars.peek() {
                    Some('=') => {
                        self.chars.next();
                        match self.chars.peek() {
                            Some('>') => {
                                self.chars.next();
                                self.simple_lexeme(LessEqualGreater, 3)
                            }
                            _ => self.simple_lexeme(LessEqualGreater, 2),
                        }
                    }
                    Some('<') => self.simple_lexeme(LessLess, 2),
                    _ => self.simple_lexeme(Less, 1),
                },
                '-' => match self.chars.peek() {
                    Some('@') => {
                        self.chars.next();
                        self.simple_lexeme(MinusAt, 2)
                    }
                    Some('=') => {
                        self.chars.next();
                        self.simple_lexeme(MinusEqual, 2)
                    }
                    Some('>') => {
                        self.chars.next();
                        self.simple_lexeme(MinusGreater, 2)
                    }
                    _ => self.simple_lexeme(Minus, 1),
                },
                '%' => match self.chars.peek() {
                    Some('=') => {
                        self.chars.next();
                        self.simple_lexeme(PercentEqual, 2)
                    }
                    Some('i') => {
                        self.chars.next();
                        self.simple_lexeme(PercentLowerI, 2)
                    }
                    Some('w') => {
                        self.chars.next();
                        self.simple_lexeme(PercentLowerW, 2)
                    }
                    Some('x') => {
                        self.chars.next();
                        self.simple_lexeme(PercentLowerX, 2)
                    }
                    Some('I') => {
                        self.chars.next();
                        self.simple_lexeme(PercentUpperI, 2)
                    }
                    Some('W') => {
                        self.chars.next();
                        self.simple_lexeme(PercentUpperW, 2)
                    }
                    _ => self.simple_lexeme(Percent, 1),
                },
                '|' => match self.chars.peek() {
                    Some('=') => self.simple_lexeme(PipeEqual, 2),
                    Some('|') => {
                        self.chars.next();
                        match self.chars.peek() {
                            Some('=') => {
                                self.chars.next();
                                self.simple_lexeme(PipePipeEqual, 3)
                            }
                            _ => self.simple_lexeme(PipePipe, 2),
                        }
                    }
                    _ => self.simple_lexeme(Pipe, 1),
                },
                '+' => match self.chars.peek() {
                    Some('@') => {
                        self.chars.next();
                        self.simple_lexeme(PlusAt, 2)
                    }
                    Some('=') => {
                        self.chars.next();
                        self.simple_lexeme(PlusEqual, 2)
                    }
                    _ => self.simple_lexeme(Plus, 1),
                },
                '/' => match self.chars.peek() {
                    Some('=') => {
                        self.chars.next();
                        self.simple_lexeme(SlashEqual, 2)
                    }
                    _ => self.simple_lexeme(Slash, 1),
                },
                '*' => match self.chars.peek() {
                    Some('=') => self.simple_lexeme(StarEqual, 2),
                    Some('*') => {
                        self.chars.next();
                        match self.chars.peek() {
                            Some('=') => {
                                self.chars.next();
                                self.simple_lexeme(StarStarEqual, 3)
                            }
                            _ => self.simple_lexeme(StarStar, 2),
                        }
                    }
                    _ => self.simple_lexeme(Star, 1),
                },
                '~' => match self.chars.peek() {
                    Some('@') => {
                        self.chars.next();
                        self.simple_lexeme(TildeOrTildeAt, 2)
                    }
                    _ => self.simple_lexeme(TildeOrTildeAt, 1),
                },
                ';' => self.simple_lexeme(Semicolon, 1),
                c if is_identifier_start(c) => self.identifier_or_keyword(c),
                _ => panic!(),
            },
        };
        todo!()
    }

    /// Lexes a character literal of the form `?<CHAR>`
    /// TODO: Allow remaining escape sequences. See Prism `lex_question_mark`
    /// Pre: `?` has been consumed.
    ///     `self.chars.peek()` is some non-whitespace character.
    fn character_literal(&mut self) -> Lexeme {
        match self.chars.peek() {
            Some('\\') => match self.chars.peek() {
                None => panic!(),
                Some('\\') | Some('n') | Some('t') => self.simple_lexeme(CharacterLiteral, 3),
                _ => panic!(),
            }
            Some(c) if !c.is_whitespace() => {
                self.chars.next();
                self.simple_lexeme(CharacterLiteral, 2)
            },
            _ => unreachable!()
        }
    }

    /// Lexes an identifier or a keyword starting with one of:
    /// - A lowercase letter
    /// - `_`
    /// Pre: `first_character` has been consumed.
    fn identifier_or_keyword(&mut self, first_character: char) -> Lexeme {
        // Strategy taken from Robert Nystrom -- Crafting Interpreters chapter 16
        // We use nested cases until we reach non-ambiguity between keywords.
        // Then we check against that keyword in particular.

        // len includes 1 for `first_character`
        let mut len = match first_character {
            'a' => match self.chars.peek() {
                Some('l') => match self.check_rest_of_keyword("lias", Alias, 5) {
                    Ok(kw) => return kw,
                    Err(consumed) => consumed + 1, // 'l' wasn't consumed by us.
                },
                Some('n') => match self.check_rest_of_keyword("nd", Begin, 3) {
                    Ok(kw) => return kw,
                    Err(consumed) => consumed + 1,
                },
                _ => 1,
            },
            'b' => match self.chars.peek() {
                Some('e') => match self.check_rest_of_keyword("egin", Begin, 5) {
                    Ok(kw) => return kw,
                    Err(consumed) => consumed + 1,
                },
                Some('r') => match self.check_rest_of_keyword("reak", Break, 5) {
                    Ok(kw) => return kw,
                    Err(consumed) => consumed + 1,
                },
                _ => 1,
            },
            'c' => match self.chars.peek() {
                Some('c') => match self.check_rest_of_keyword("ase", Case, 4) {
                    Ok(kw) => return kw,
                    Err(consumed) => consumed + 1,
                },
                Some('l') => match self.check_rest_of_keyword("lass", Class, 5) {
                    Ok(kw) => return kw,
                    Err(consumed) => consumed + 1,
                },
                _ => 1,
            },
            'd' => match self.chars.peek() {
                Some('e') => match self.check_rest_of_keyword("f", Def, 3) {
                    Ok(kw) => return kw,
                    Err(consumed) => consumed + 1,
                },
                Some('o') => {
                    self.chars.next();
                    match self.chars.peek() {
                        Some(&c) if is_identifier_char(c) => 2,
                        _ => {
                            self.chars.next();
                            return self.simple_lexeme(Do, 2);
                        }
                    }
                }
                _ => 1,
            },
            'e' => match self.chars.peek() {
                Some('l') => {
                    self.chars.next();
                    match self.chars.peek() {
                        Some('s') => {
                            self.chars.next();
                            match self.chars.peek() {
                                Some('e') => {
                                    self.chars.next();
                                    match self.chars.peek() {
                                        Some('i') => {
                                            match self.check_rest_of_keyword("f", Elsif, 6) {
                                                Ok(kw) => return kw,
                                                Err(consumed) => consumed + 4,
                                            }
                                        }
                                        Some(&c) if is_identifier_char(c) => 4,
                                        _ => {
                                            self.chars.next();
                                            return self.simple_lexeme(Else, 4);
                                        }
                                    }
                                }
                                _ => 3,
                            }
                        }
                        _ => 2,
                    }
                }
                Some('n') => match self.chars.peek() {
                    Some('d') => {
                        self.chars.next();
                        match self.chars.peek() {
                            Some(&c) if is_identifier_char(c) => 3,
                            _ => {
                                self.chars.next();
                                return self.simple_lexeme(End, 3);
                            }
                        }
                    }
                    Some('s') => match self.check_rest_of_keyword("ensure", Ensure, 6) {
                        Ok(kw) => return kw,
                        Err(consumed) => consumed + 1,
                    },
                    _ => 1,
                },
                _ => 1,
            },
            'f' => match self.check_rest_of_keyword("false", False, 5) {
                Ok(kw) => return kw,
                Err(consumed) => consumed + 1,
            },
            'i' => match self.check_rest_of_keyword("in", In, 2) {
                Ok(kw) => return kw,
                Err(consumed) => consumed + 1,
            },
            'n' => match self.chars.peek() {
                Some('i') => match self.check_rest_of_keyword("nil", Nil, 3) {
                    Ok(kw) => return kw,
                    Err(consumed) => consumed + 1, // 'l' wasn't consumed by us.
                },
                Some('o') => match self.check_rest_of_keyword("not", Not, 3) {
                    Ok(kw) => return kw,
                    Err(consumed) => consumed + 1,
                },
                _ => 1,
            },
            'o' => match self.check_rest_of_keyword("or", Or, 2) {
                Ok(kw) => return kw,
                Err(consumed) => consumed + 1,
            },
            'r' => match self.chars.peek() {
                Some('e') => {
                    self.chars.next();
                    match self.chars.peek() {
                        Some('d') => match self.check_rest_of_keyword("o", Redo, 4) {
                            Ok(kw) => return kw,
                            Err(consumed) => consumed + 2,
                        },
                        Some('s') => match self.check_rest_of_keyword("cue", Rescue, 6) {
                            Ok(kw) => return kw,
                            Err(consumed) => consumed + 2,
                        },
                        Some('t') => {
                            self.chars.next();
                            match self.chars.peek() {
                                Some('r') => match self.check_rest_of_keyword("y", Retry, 5) {
                                    Ok(kw) => return kw,
                                    Err(consumed) => consumed + 3,
                                },
                                Some('u') => match self.check_rest_of_keyword("rn", Return, 6) {
                                    Ok(kw) => return kw,
                                    Err(consumed) => consumed + 3,
                                },
                                _ => 3,
                            }
                        }
                        _ => 2,
                    }
                }
                _ => 1,
            },
            's' => match self.chars.peek() {
                Some('e') => match self.check_rest_of_keyword("lf", SelfKeyword, 4) {
                    Ok(kw) => return kw,
                    Err(consumed) => consumed + 1, // 'l' wasn't consumed by us.
                },
                Some('u') => match self.check_rest_of_keyword("per", Super, 5) {
                    Ok(kw) => return kw,
                    Err(consumed) => consumed + 1,
                },
                _ => 1,
            },
            't' => match self.chars.peek() {
                Some('h') => match self.check_rest_of_keyword("en", Then, 4) {
                    Ok(kw) => return kw,
                    Err(consumed) => consumed + 1, // 'l' wasn't consumed by us.
                },
                Some('r') => match self.check_rest_of_keyword("ue", True, 4) {
                    Ok(kw) => return kw,
                    Err(consumed) => consumed + 1,
                },
                _ => 1,
            },
            'u' => match self.chars.peek() {
                Some('n') => {
                    self.chars.next();
                    match self.chars.peek() {
                        Some('d') => match self.check_rest_of_keyword("ef", Undef, 5) {
                            Ok(kw) => return kw,
                            Err(consumed) => consumed + 2,
                        },
                        Some('l') => match self.check_rest_of_keyword("ess", Unless, 6) {
                            Ok(kw) => return kw,
                            Err(consumed) => consumed + 2,
                        },
                        Some('t') => match self.check_rest_of_keyword("il", Until, 5) {
                            Ok(kw) => return kw,
                            Err(consumed) => consumed + 2,
                        },
                        _ => 2,
                    }
                }
                _ => 1,
            },
            'w' => match self.chars.peek() {
                Some('h') => {
                    self.chars.next();
                    match self.chars.peek() {
                        Some('e') => match self.check_rest_of_keyword("n", When, 4) {
                            Ok(kw) => return kw,
                            Err(consumed) => consumed + 2,
                        },
                        Some('i') => match self.check_rest_of_keyword("le", While, 5) {
                            Ok(kw) => return kw,
                            Err(consumed) => consumed + 2,
                        },
                        _ => 2,
                    }
                }
                _ => 1,
            },
            'y' => match self.check_rest_of_keyword("ield", Yield, 5) {
                Ok(kw) => return kw,
                Err(consumed) => consumed + 1,
            },
            '_' => match self.chars.peek() {
                Some('_') => {
                    self.chars.next();
                    match self.chars.peek() {
                        Some('E') => {
                            match self.check_rest_of_keyword("NCODING__", UnderscoreEncoding, 12) {
                                Ok(kw) => return kw,
                                Err(consumed) => consumed + 2,
                            }
                        }
                        Some('F') => match self.check_rest_of_keyword("ILE__", UnderscoreFile, 8) {
                            Ok(kw) => return kw,
                            Err(consumed) => consumed + 2,
                        },
                        Some('L') => match self.check_rest_of_keyword("INE__", UnderscoreLine, 8) {
                            Ok(kw) => return kw,
                            Err(consumed) => consumed + 2,
                        },
                        _ => 2,
                    }
                }
                _ => 1,
            },
            _ => 1,
        };

        // This is not a keyword, so consume until we reach a non-identifier character.
        loop {
            match self.chars.peek() {
                Some(&c) if is_identifier_char(c) => {
                    len += 1;
                    self.chars.next();
                }
                _ => return self.simple_lexeme(Identifier, len),
            }
        }
    }

    /// Checks if the rest of this keyword/const/identifier is the given keyword.
    fn check_rest_of_keyword(
        &mut self,
        expected: &str,
        kind: LexemeKind,
        len: u64,
    ) -> Result<Lexeme, u64> {
        let mut consumed = 0;
        for expected_char in expected.chars() {
            match self.chars.peek() {
                Some(&actual_char) if expected_char == actual_char => {
                    self.chars.next();
                    consumed += 1;
                }
                _ => return Err(consumed),
            }
        }

        // We've reached the end of the keyword text.
        // Check whether there's any additional characters in this keyword/const/identifier.
        match self.chars.peek() {
            Some(&c) if is_identifier_char(c) => Err(consumed),
            _ => Ok(self.simple_lexeme(kind, len)),
        }
    }

    /// Pre: '\n' has been consumed.
    fn newline(&mut self) -> Lexeme {
        let start_line = self.line;
        let start_col = self.col;
        self.line += 1;
        self.col = 0;
        Lexeme::new(Newline, start_line, start_col, self.line, 0)
    }

    /// Updates coordinates and returns a simple (no linebreaks) lexeme.
    /// Pre:
    ///     Coords at start of lexeme.
    ///     All lexeme characters have been consumed.
    fn simple_lexeme(&mut self, kind: LexemeKind, len: u64) -> Lexeme {
        // NB: Compound punctuation can't be separated by whitespace.
        //  `& =` -> `AMPERSAND, EQUALS`
        //  `&=` -> `AMPERSAND_EQUALS`
        let start_col = self.col;
        self.col += len;
        Lexeme::new(kind, self.line, start_col, self.line, self.col)
    }

    /// Skip non-newline whitespace, incrementing `self.col`.
    fn skip_whitespace(&mut self) {
        while self
            .chars
            .peek()
            .is_some_and(|&c| c.is_whitespace() && c != '\n')
        {
            self.col += 1;
            self.chars.next();
        }
    }
}

/// Is `c` alphabetic or an underscore?
/// TODO: I think Ruby allows certain other chars in UTF-8.
///     It also has many other encodings I'm going to ignore.
fn is_identifier_start(c: char) -> bool {
    c == '_' || (c.is_alphabetic() && c.is_lowercase())
}

fn is_identifier_char(c: char) -> bool {
    c.is_alphanumeric() || c == '?'
}

#[cfg(test)]
mod tests {
    use super::*;
    use LexemeKind::*;

    #[test]
    pub fn empty() {
        let text = "";
        let expected: Vec<Lexeme> = vec![Lexeme::new(Eof, 1, 0, 1, 0)];
        let actual = Lexer::new(text).lex();
        assert_eq!(expected, actual);

        let text = "     ";
        let expected: Vec<Lexeme> = vec![];
        let actual = Lexer::new(text).lex();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn keywords() {
        {
            let text = "nil";
            let expected: Vec<Lexeme> =
                vec![Lexeme::new(Nil, 1, 0, 1, 3), Lexeme::new(Eof, 1, 3, 1, 3)];

            let test_tokens = Lexer::new(text).lex();

            assert_eq!(expected, test_tokens);
        }
    }
}
