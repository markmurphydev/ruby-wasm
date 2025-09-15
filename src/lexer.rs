use crate::lexeme::{CharDifference, CharIdx, Col, Lexeme, LexemeKind, Line};
use std::str::Chars;

use LexemeKind::*;
use crate::lexeme;

/// Lexes Ruby program input.
/// Attempts to produce identical output to the Prism lexer.
/// NOTE: If we find ourselves adding state to this struct, consider whether
/// it would be better added as parser-driven input to the lex method
pub struct Lexer<'text> {
    text: &'text str,
    /// Peekable iterator of `(char_idx, char)`
    iter: LexerIter<'text>,
    /// Have we lexed Eof?
    lexed_eof: bool,
}

impl<'text> Lexer<'text> {
    pub fn new(text: &'text str) -> Self {
        Self {
            text,
            iter: LexerIter::from(text.chars()),
            lexed_eof: false,
        }
    }

    /// Consume and return a single lexeme
    pub fn lex(&mut self) -> Lexeme {
        // Strategy: For punctuation, simple matching.

        if self.lexed_eof {
            panic!();
        }

        // Prism doesn't include whitespace in token width.
        self.skip_whitespace();

        let lexeme = match self.iter.next() {
            // Prism EOF is 0-width.
            None => {
                self.lexed_eof = true;
                Lexeme::new(Eof, self.iter.highest_idx(), CharDifference(0))
            },
            Some((start_idx, c)) => match c {
                '\n' => Lexeme::new(Newline, start_idx, CharDifference(1)),

                '#' => {
                    let final_idx = self.skip_to_next_line(start_idx);
                    Lexeme::new(InlineComment, start_idx, len_inclusive(start_idx, final_idx))
                }

                // '?' can be the start of a character literal, or a ternary operator
                '?' => match self.iter.peek() {
                    None => Lexeme::new(Question, start_idx, CharDifference(1)),
                    Some((_, c)) if c.is_whitespace() => Lexeme::new(Question, start_idx, CharDifference(1)),
                    _ => self.character_literal(start_idx),
                },

                c if c.is_ascii_digit() => self.integer_or_float(start_idx),

                '\'' => {
                    self.iter.next();
                    self.single_quote_string(start_idx)
                }

                '@' => {
                    self.iter.next();
                    match self.iter.peek() {
                        Some((_, '@')) => {
                            self.iter.next();
                            self.class_variable(start_idx)
                        }
                        Some((_, c)) if !c.is_whitespace() => self.instance_variable(start_idx),
                        Some(_) => panic!("Bare `@`"),
                        None => panic!("Bare `@`"),
                    }
                }

                // Punctuation
                // TODO: This can definitely be replaced by a macro that generates the match chain.
                //  Only problem would be losing error reporting.
                //  Which is one reason not to use regexes in the first place...
                //  Ditto keywords tho
                '&' => match self.iter.peek() {
                    Some((_, '&')) => {
                        self.iter.next();
                        match self.iter.peek() {
                            Some((_, '=')) => {
                                self.iter.next();
                                Lexeme::new(AmpersandAmpersandEqual, start_idx, CharDifference(3))
                            }
                            _ => Lexeme::new(AmpersandAmpersand, start_idx, CharDifference(2)),
                        }
                    }
                    Some((_, '=')) => {
                        self.iter.next();
                        Lexeme::new(AmpersandEqual, start_idx, CharDifference(2))
                    }
                    Some((_, '.')) => {
                        self.iter.next();
                        Lexeme::new(AmpersandPeriod, start_idx, CharDifference(2))
                    }
                    _ => Lexeme::new(Ampersand, start_idx, CharDifference(1)),
                },
                '`' => Lexeme::new(Backtick, start_idx, CharDifference(1)),
                '!' => match self.iter.peek() {
                    Some((_, '=')) => {
                        self.iter.next();
                        Lexeme::new(BangEqual, start_idx, CharDifference(2))
                    }
                    Some((_, '~')) => {
                        self.iter.next();
                        Lexeme::new(BangTilde, start_idx, CharDifference(2))
                    }
                    _ => Lexeme::new(Bang, start_idx, CharDifference(1)),
                },
                '{' => Lexeme::new(BraceLeft, start_idx, CharDifference(1)),
                '}' => Lexeme::new(BraceRight, start_idx, CharDifference(1)),
                '[' => match self.iter.peek() {
                    Some((_, ']')) => {
                        self.iter.next();
                        match self.iter.peek() {
                            Some((_, '=')) => {
                                self.iter.next();
                                Lexeme::new(BracketLeftRightEqual, start_idx, CharDifference(3))
                            }
                            _ => Lexeme::new(BracketLeftRight, start_idx, CharDifference(2)),
                        }
                    }
                    _ => Lexeme::new(BracketLeft, start_idx, CharDifference(1)),
                },
                ']' => Lexeme::new(BracketRight, start_idx, CharDifference(1)),
                '^' => match self.iter.peek() {
                    Some((_, '=')) => {
                        self.iter.next();
                        Lexeme::new(CaretEqual, start_idx, CharDifference(2))
                    }
                    _ => Lexeme::new(Caret, start_idx, CharDifference(1)),
                },
                ':' => match self.iter.peek() {
                    Some((_, ':')) => {
                        self.iter.next();
                        Lexeme::new(ColonColon, start_idx, CharDifference(2))
                    }
                    _ => Lexeme::new(Colon, start_idx, CharDifference(1)),
                },
                ',' => Lexeme::new(Comma, start_idx, CharDifference(1)),
                '.' => match self.iter.peek() {
                    Some((_, '.')) => {
                        self.iter.next();
                        match self.iter.peek() {
                            Some((_, '.')) => {
                                self.iter.next();
                                Lexeme::new(DotDotDot, start_idx, CharDifference(3))
                            }
                            _ => Lexeme::new(DotDot, start_idx, CharDifference(2)),
                        }
                    }
                    _ => Lexeme::new(Dot, start_idx, CharDifference(1)),
                },
                '=' => match self.iter.peek() {
                    Some((_, '=')) => {
                        self.iter.next();
                        match self.iter.peek() {
                            Some((_, '=')) => {
                                self.iter.next();
                                Lexeme::new(EqualEqualEqual, start_idx, CharDifference(3))
                            }
                            _ => Lexeme::new(EqualEqual, start_idx, CharDifference(2)),
                        }
                    }
                    Some((_, '>')) => Lexeme::new(EqualGreater, start_idx, CharDifference(2)),
                    Some((_, '~')) => Lexeme::new(EqualTilde, start_idx, CharDifference(2)),
                    _ => Lexeme::new(Equal, start_idx, CharDifference(1)),
                },
                '>' => match self.iter.peek() {
                    Some((_, '>')) => {
                        self.iter.next();
                        match self.iter.peek() {
                            Some((_, '=')) => {
                                self.iter.next();
                                Lexeme::new(GreaterGreaterEqual, start_idx, CharDifference(3))
                            }
                            _ => Lexeme::new(GreaterGreater, start_idx, CharDifference(2)),
                        }
                    }
                    Some((_, '=')) => Lexeme::new(GreaterEqual, start_idx, CharDifference(2)),
                    _ => Lexeme::new(Greater, start_idx, CharDifference(1)),
                },
                '<' => match self.iter.peek() {
                    Some((_, '=')) => {
                        self.iter.next();
                        match self.iter.peek() {
                            Some((_, '>')) => {
                                self.iter.next();
                                Lexeme::new(LessEqualGreater, start_idx, CharDifference(3))
                            }
                            _ => Lexeme::new(LessEqualGreater, start_idx, CharDifference(2)),
                        }
                    }
                    Some((_, '<')) => Lexeme::new(LessLess, start_idx, CharDifference(2)),
                    _ => Lexeme::new(Less, start_idx, CharDifference(1)),
                },
                '-' => match self.iter.peek() {
                    Some((_, '@')) => {
                        self.iter.next();
                        Lexeme::new(MinusAt, start_idx, CharDifference(2))
                    }
                    Some((_, '=')) => {
                        self.iter.next();
                        Lexeme::new(MinusEqual, start_idx, CharDifference(2))
                    }
                    Some((_, '>')) => {
                        self.iter.next();
                        Lexeme::new(MinusGreater, start_idx, CharDifference(2))
                    }
                    _ => Lexeme::new(Minus, start_idx, CharDifference(1)),
                },
                '%' => match self.iter.peek() {
                    Some((_, '=')) => {
                        self.iter.next();
                        Lexeme::new(PercentEqual, start_idx, CharDifference(2))
                    }
                    Some((_, 'i')) => {
                        self.iter.next();
                        Lexeme::new(PercentLowerI, start_idx, CharDifference(2))
                    }
                    Some((_, 'w')) => {
                        self.iter.next();
                        Lexeme::new(PercentLowerW, start_idx, CharDifference(2))
                    }
                    Some((_, 'x')) => {
                        self.iter.next();
                        Lexeme::new(PercentLowerX, start_idx, CharDifference(2))
                    }
                    Some((_, 'I')) => {
                        self.iter.next();
                        Lexeme::new(PercentUpperI, start_idx, CharDifference(2))
                    }
                    Some((_, 'W')) => {
                        self.iter.next();
                        Lexeme::new(PercentUpperW, start_idx, CharDifference(2))
                    }
                    _ => Lexeme::new(Percent, start_idx, CharDifference(1)),
                },
                '|' => match self.iter.peek() {
                    Some((_, '=')) => Lexeme::new(PipeEqual, start_idx, CharDifference(2)),
                    Some((_, '|')) => {
                        self.iter.next();
                        match self.iter.peek() {
                            Some((_, '=')) => {
                                self.iter.next();
                                Lexeme::new(PipePipeEqual, start_idx, CharDifference(3))
                            }
                            _ => Lexeme::new(PipePipe, start_idx, CharDifference(2)),
                        }
                    }
                    _ => Lexeme::new(Pipe, start_idx, CharDifference(1)),
                },
                '+' => match self.iter.peek() {
                    Some((_, '@')) => {
                        self.iter.next();
                        Lexeme::new(PlusAt, start_idx, CharDifference(2))
                    }
                    Some((_, '=')) => {
                        self.iter.next();
                        Lexeme::new(PlusEqual, start_idx, CharDifference(2))
                    }
                    _ => Lexeme::new(Plus, start_idx, CharDifference(1)),
                },
                '/' => match self.iter.peek() {
                    Some((_, '=')) => {
                        self.iter.next();
                        Lexeme::new(SlashEqual, start_idx, CharDifference(2))
                    }
                    _ => Lexeme::new(Slash, start_idx, CharDifference(1)),
                },
                '*' => match self.iter.peek() {
                    Some((_, '=')) => Lexeme::new(StarEqual, start_idx, CharDifference(2)),
                    Some((_, '*')) => {
                        self.iter.next();
                        match self.iter.peek() {
                            Some((_, '=')) => {
                                self.iter.next();
                                Lexeme::new(StarStarEqual, start_idx, CharDifference(3))
                            }
                            _ => Lexeme::new(StarStar, start_idx, CharDifference(2)),
                        }
                    }
                    _ => Lexeme::new(Star, start_idx, CharDifference(1)),
                },
                '~' => match self.iter.peek() {
                    Some((_, '@')) => {
                        self.iter.next();
                        Lexeme::new(TildeOrTildeAt, start_idx, CharDifference(2))
                    }
                    _ => Lexeme::new(TildeOrTildeAt, start_idx, CharDifference(1)),
                },
                ';' => Lexeme::new(Semicolon, start_idx, CharDifference(1)),
                c if c.is_ascii_uppercase() => self.constant(start_idx),
                c if is_identifier_start(c) => self.identifier_or_keyword(start_idx, c),
                _ => panic!(),
            },
        };

        lexeme
    }

    /// Lexes a character literal of the form `?<CHAR>`
    /// TODO: Allow remaining escape sequences. See Prism `lex_question_mark`
    /// Pre: `?` has been consumed.
    ///     `self.chars.peek()` is some non-whitespace character.
    fn character_literal(&mut self, start_idx: CharIdx) -> Lexeme {
        match self.iter.peek() {
            Some((_, '\\')) => match self.iter.peek() {
                None => panic!(),
                Some((_, '\\')) | Some((_, 'n')) | Some((_, 't')) => Lexeme::new(CharacterLiteral, start_idx, CharDifference(3)),
                _ => panic!(),
            },
            Some((_, c)) if !c.is_whitespace() => {
                self.iter.next();
                Lexeme::new(CharacterLiteral, start_idx, CharDifference(2))
            }
            _ => unreachable!(),
        }
    }

    fn integer_or_float(&mut self, start_idx: CharIdx) -> Lexeme {
        let mut len = 1;
        loop {
            match self.iter.peek() {
                // Integers and floats can contain `_`, but can't end on it.
                // Floats also can't have an underscore directly before the period.
                Some((_, '_')) => {
                    len += 1;
                    self.iter.next();
                    match self.iter.peek() {
                        Some((_, c)) if c.is_ascii_digit() => continue,
                        _ => panic!(),
                    }
                }
                Some((_, '.')) => {
                    len += 1;
                    break;
                }
                Some((_, c)) if c.is_ascii_digit() => {
                    len += 1;
                    self.iter.next();
                }
                Some((_, c)) if c.is_whitespace() => return self.integer_literal(start_idx, CharDifference(len)),
                None => return self.integer_literal(start_idx, CharDifference(len)),
                _ => panic!(),
            }
        }

        // This is a float.
        loop {
            match self.iter.peek() {
                // Floats can contain `_` in the decimal part, but can't end on it.
                Some((_, '_')) => {
                    len += 1;
                    self.iter.next();
                    match self.iter.peek() {
                        Some((_, c)) if c.is_ascii_digit() => continue,
                        _ => panic!(),
                    }
                }
                Some((_, c)) if c.is_ascii_digit() => {
                    len += 1;
                    self.iter.next();
                }
                Some((_, c)) if c.is_whitespace() => return Lexeme::new(FloatLiteral, start_idx, CharDifference(len)),
                _ => panic!(),
            }
        }
    }

    /// Lexes a string surrounded by single quotes.
    /// Allows only the escape characters `\'` and `\\`
    /// Pre: `'` has been consumed.
    fn single_quote_string(&mut self, start_idx: CharIdx) -> Lexeme {
        loop {
            match self.iter.next() {
                None => panic!("Unterminated single-quote string literal"),
                // Consume `\'` without returning.
                Some((_, '\\')) => {
                    match self.iter.next() {
                        None => panic!("Unterminated single-quote string literal"),
                        Some(_) => (),
                    }
                }
                Some((idx, '\'')) => return Lexeme::new(SingleQuoteStringLiteral, start_idx, len_inclusive(start_idx, idx)),
                Some(_) => (),
            };
        }
    }

    /// Lexes an instance variable of the form `@<IDENTIFIER>`
    /// Pre: `@` has been consumed.
    /// TODO: This identifier-recognition code is duplicated like 4 times.
    ///     But I'm afraid it might eventually be different between identifier types?
    ///     Ruby _would_ do something obnoxious like that.
    fn instance_variable(&mut self, start_idx: CharIdx) -> Lexeme {
        let mut len = 1;

        loop {
            match self.iter.peek() {
                Some((_, c)) if is_identifier_char(c) => {
                    len += 1;
                    self.iter.next();
                }
                _ => return Lexeme::new(InstanceVariable, start_idx, CharDifference(len)),
            }
        }
    }

    /// Lexes a class variable of the form `@@<IDENTIFIER>`
    /// Pre: `@@` has been consumed.
    fn class_variable(&mut self, start_idx: CharIdx) -> Lexeme {
        let mut len = 1;

        loop {
            match self.iter.peek() {
                Some((_, c)) if is_identifier_char(c) => {
                    len += 1;
                    self.iter.next();
                }
                _ => return Lexeme::new(ClassVariable, start_idx, CharDifference(len)),
            }
        }
    }

    /// Lexes a constant of the form `[A-Z]<IDENTIFIER>`
    /// Pre: Initial uppercase character has been consumed.
    fn constant(&mut self, start_idx: CharIdx) -> Lexeme {
        let mut len = 1;

        loop {
            match self.iter.peek() {
                Some((_, c)) if is_identifier_char(c) => {
                    len += 1;
                    self.iter.next();
                }
                _ => return Lexeme::new(Constant, start_idx, CharDifference(len)),
            }
        }
    }

    /// Lexes an identifier or a keyword starting with one of:
    /// - A lowercase letter
    /// - `_`
    /// Pre: `first_character` has been consumed.
    fn identifier_or_keyword(&mut self, start_idx: CharIdx, first_character: char) -> Lexeme {
        // Strategy taken from Robert Nystrom -- Crafting Interpreters chapter 16
        // We use nested cases until we reach non-ambiguity between keywords.
        // Then we check against that keyword in particular.

        // len includes 1 for `first_character`
        let maybe_lexeme = match first_character {
            'a' => match self.iter.peek() {
                Some((_, 'l')) => self.check_rest_of_keyword("lias", Alias, start_idx),
                Some((_, 'n')) =>  self.check_rest_of_keyword("nd", Begin, start_idx),
                _ => None,
            },
            'b' => match self.iter.peek() {
                Some((_, 'e')) =>  self.check_rest_of_keyword("egin", Begin, start_idx) ,
                Some((_, 'r')) =>  self.check_rest_of_keyword("reak", Break, start_idx) ,
                _ => None,
            },
            'c' => match self.iter.peek() {
                Some((_, 'a')) =>  self.check_rest_of_keyword("ase", Case, start_idx) ,
                Some((_, 'l')) =>  self.check_rest_of_keyword("lass", Class, start_idx) ,
                _ => None,
            },
            'd' => match self.iter.peek() {
                Some((_, 'e')) =>  self.check_rest_of_keyword("ef", Def, start_idx) ,
                Some((_, 'o')) => {
                    self.iter.next();
                    match self.iter.peek() {
                        Some((_, c)) if is_identifier_char(c) => None,
                        _ => {
                            self.iter.next();
                            Some(Lexeme::new(Do, start_idx, CharDifference(2)))
                        }
                    }
                }
                _ => None,
            },
            'e' => match self.iter.peek() {
                Some((_, 'l')) => {
                    self.iter.next();
                    match self.iter.peek() {
                        Some((_, 's')) => {
                            self.iter.next();
                            match self.iter.peek() {
                                Some((_, 'e')) => {
                                    self.iter.next();
                                    match self.iter.peek() {
                                        Some((_, 'i')) => {
                                             self.check_rest_of_keyword("if", Elsif, start_idx)
                                        }
                                        Some((_, c)) if is_identifier_char(c) => None,
                                        _ => {
                                            self.iter.next();
                                            Some(Lexeme::new(Else, start_idx, CharDifference(4)))
                                        }
                                    }
                                }
                                _ => None,
                            }
                        }
                        _ => None,
                    }
                }
                Some((_, 'n')) => match self.iter.peek() {
                    Some((_, 'd')) => {
                        self.iter.next();
                        match self.iter.peek() {
                            Some((_, c)) if is_identifier_char(c) => None,
                            _ => {
                                self.iter.next();
                                return Lexeme::new(End, start_idx, CharDifference(3));
                            }
                        }
                    }
                    Some((_, 's')) =>  self.check_rest_of_keyword("sure", Ensure, start_idx),
                    _ => None,
                },
                _ => None,
            },
            'f' =>  self.check_rest_of_keyword("alse", False, start_idx) ,
            'i' =>  self.check_rest_of_keyword("in", In, start_idx) ,
            'n' => match self.iter.peek() {
                Some((_, 'i')) =>  self.check_rest_of_keyword("il", Nil, start_idx) ,
                Some((_, 'o')) =>  self.check_rest_of_keyword("ot", Not, start_idx) ,
                _ => None,
            },
            'o' =>  self.check_rest_of_keyword("or", Or, start_idx) ,
            'r' => match self.iter.peek() {
                Some((_, 'e')) => {
                    self.iter.next();
                    match self.iter.peek() {
                        Some((_, 'd')) =>  self.check_rest_of_keyword("do", Redo, start_idx) ,
                        Some((_, 's')) =>  self.check_rest_of_keyword("scue", Rescue, start_idx) ,
                        Some((_, 't')) => {
                            self.iter.next();
                            match self.iter.peek() {
                                Some((_, 'r')) =>  self.check_rest_of_keyword("ry", Retry, start_idx) ,
                                Some((_, 'u')) =>  self.check_rest_of_keyword("urn", Return, start_idx) ,
                                _ => None,
                            }
                        }
                        _ => None,
                    }
                }
                _ => None,
            },
            's' => match self.iter.peek() {
                Some((_, 'e')) =>  self.check_rest_of_keyword("elf", SelfKeyword, start_idx) ,
                Some((_, 'u')) =>  self.check_rest_of_keyword("uper", Super, start_idx) ,
                _ => None,
            },
            't' => match self.iter.peek() {
                Some((_, 'h')) =>  self.check_rest_of_keyword("hen", Then, start_idx) ,
                Some((_, 'r')) =>  self.check_rest_of_keyword("rue", True, start_idx) ,
                _ => None,
            },
            'u' => match self.iter.peek() {
                Some((_, 'n')) => {
                    self.iter.next();
                    match self.iter.peek() {
                        Some((_, 'd')) =>  self.check_rest_of_keyword("def", Undef, start_idx) ,
                        Some((_, 'l')) =>  self.check_rest_of_keyword("less", Unless, start_idx) ,
                        Some((_, 't')) =>  self.check_rest_of_keyword("til", Until, start_idx) ,
                        _ => None,
                    }
                }
                _ => None,
            },
            'w' => match self.iter.peek() {
                Some((_, 'h')) => {
                    self.iter.next();
                    match self.iter.peek() {
                        Some((_, 'e')) =>  self.check_rest_of_keyword("en", When, start_idx) ,
                        Some((_, 'i')) =>  self.check_rest_of_keyword("ile", While, start_idx) ,
                        _ => None,
                    }
                }
                _ => None,
            },
            'y' => self.check_rest_of_keyword("ield", Yield, start_idx) ,
            '_' => match self.iter.peek() {
                Some((_, '_')) => {
                    self.iter.next();
                    match self.iter.peek() {
                        Some((_, 'E')) => {
                             self.check_rest_of_keyword("ENCODING__", UnderscoreEncoding, start_idx)
                        }
                        Some((_, 'F')) => {
                             self.check_rest_of_keyword("FILE__", UnderscoreFile, start_idx)
                        }
                        Some((_, 'L')) => {
                             self.check_rest_of_keyword("LINE__", UnderscoreLine, start_idx)
                        }
                        _ => None,
                    }
                }
                _ => None,
            },
            _ => None,
        };
        if let Some(lexeme) = maybe_lexeme {
            return lexeme;
        }

        // This is not a keyword, so consume until we reach a non-identifier character.
        loop {
            match self.iter.peek() {
                Some((_, c)) if is_identifier_char(c) => {
                    self.iter.next();
                }
                Some((idx, _)) => {
                    return Lexeme::new(Identifier, start_idx, len_inclusive(start_idx, idx))
                }
                None => {
                    let idx = self.iter.highest_idx();
                    return Lexeme::new(Identifier, start_idx, len_inclusive(start_idx, idx))
                },
            }
        }
    }

    /// Checks if the rest of this keyword/const/identifier is the given keyword.
    fn check_rest_of_keyword(
        &mut self,
        expected: &str,
        kind: LexemeKind,
        start_idx: CharIdx,
    ) -> Option<Lexeme> {
        for expected_char in expected.chars() {
            match self.iter.peek() {
                Some((_, actual_char)) if expected_char == actual_char => {
                    self.iter.next();
                }
                _ => return None,
            }
        }

        // We've reached the end of the keyword text.
        // Check whether there's any additional characters in this keyword/const/identifier.
        match self.iter.peek() {
            Some((_, c)) if is_identifier_char(c) => None,
            Some((idx, _)) => {
                Some(Lexeme::new(kind, start_idx, len_exclusive(start_idx, idx)))
            }
            None => {
                let idx = self.iter.highest_idx();
                Some(Lexeme::new(kind, start_idx, len_inclusive(start_idx, idx)))
            },
        }
    }

    /// Skip until the start (column 0) of the next line.
    /// Returns the index of the last skipped character
    /// Pre: '#' was just consumed.
    fn skip_to_next_line(&mut self, start_idx: CharIdx) -> CharIdx {
        let mut final_idx = start_idx;
        loop {
            match self.iter.next() {
                None => return final_idx,
                Some((idx, '\n')) => {
                    return idx;
                }
                Some((idx, _)) => {
                    final_idx = idx;
                }
            }
        }
    }

    /// Skip non-newline whitespace, incrementing `self.col`.
    fn skip_whitespace(&mut self) {
        while self
            .iter
            .peek()
            .is_some_and(|(_, c)| c.is_whitespace() && c != '\n')
        {
            self.iter.next();
        }
    }

    fn integer_literal(&self, start_idx: CharIdx, len: CharDifference) -> Lexeme {
        let lexeme_text = lexeme::text_in_range(self.text, start_idx, len);
        Lexeme::new(IntegerLiteral { text: lexeme_text }, start_idx, len)
    }
}

/// Gets the length of the doubly-inclusive range `[self, rhs]`.
pub fn len_inclusive(lhs: CharIdx, rhs: CharIdx) -> CharDifference {
    assert!(lhs.0 <= rhs.0);
    CharDifference(rhs.0 - lhs.0 + 1)
}

/// Gets the length of the range `[self, rhs)`.
pub fn len_exclusive(lhs: CharIdx, rhs: CharIdx) -> CharDifference {
    assert!(lhs.0 < rhs.0);
    CharDifference(rhs.0 - lhs.0)
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

/// Peekable iterator over `(CharIdx, char)`
struct LexerIter<'a> {
    last_idx: Option<CharIdx>,
    iter: Chars<'a>,
    /// Remember a peeked value, even if it was None.
    peeked: Option<Option<(CharIdx, char)>>,
}

impl<'a> From<Chars<'a>> for LexerIter<'a> {
    fn from(value: Chars<'a>) -> Self {
        Self {
            last_idx: None,
            iter: value,
            peeked: None,
        }
    }
}

impl<'a> LexerIter<'a> {
    /// Get the highest idx produced by this iterator (from `next()` or `peek()`),
    ///     or `CharIdx(0)` if no such index has been returned.
    pub fn highest_idx(&self) -> CharIdx {
        self.last_idx.unwrap_or(CharIdx(0))
    }

    pub fn peek(&mut self) -> Option<(CharIdx, char)> {
        match self.peeked {
            Some(v) => v,
            None => {
                let v = self.next();
                self.peeked = Some(v);
                v
            }
        }
    }
}

impl<'a> Iterator for LexerIter<'a> {
    type Item = (CharIdx, char);

    fn next(&mut self) -> Option<Self::Item> {
        match self.peeked.take() {
            Some(v) => v,
            None => match self.iter.next() {
                None => None,
                Some(char) => {
                    let idx = match self.last_idx {
                        None => CharIdx(0),
                        Some(idx) => idx + CharDifference(1),
                    };
                    self.last_idx = Some(idx);
                    Some((idx, char))
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use LexemeKind::*;

    #[test]
    fn test_lexer_iter() {
        let text = "abc";
        let mut iter = LexerIter::from(text.chars());
        assert_eq!(Some((CharIdx(0), 'a')), iter.peek());
        assert_eq!(CharIdx(0), iter.highest_idx());
        assert_eq!(Some((CharIdx(0), 'a')), iter.peek());
        assert_eq!(CharIdx(0), iter.highest_idx());
        assert_eq!(Some((CharIdx(0), 'a')), iter.next());
        assert_eq!(CharIdx(0), iter.highest_idx());
        assert_eq!(Some((CharIdx(1), 'b')), iter.next());
        assert_eq!(CharIdx(1), iter.highest_idx());
        assert_eq!(Some((CharIdx(2), 'c')), iter.peek());
        assert_eq!(CharIdx(2), iter.highest_idx());
        assert_eq!(Some((CharIdx(2), 'c')), iter.next());
        assert_eq!(CharIdx(2), iter.highest_idx());
        assert_eq!(None, iter.peek());
        assert_eq!(CharIdx(2), iter.highest_idx());
        assert_eq!(None, iter.next());
        assert_eq!(CharIdx(2), iter.highest_idx());
    }

    /// This only works so long as the lexer can be driven independently of the parser.
    fn lex_to_eof(text: &str) -> Vec<Lexeme> {
        let mut lexemes = vec![];
        let mut lexer = Lexer::new(text);
        loop {
            let lexeme = lexer.lex();
            let eof= lexeme.kind == Eof;
            lexemes.push(lexeme);
            if eof {
                return lexemes;
            }
        }
    }

    #[test]
    pub fn empty() {
        let text = "";
        let expected: Vec<Lexeme> = vec![Lexeme::new(Eof, CharIdx(0), CharDifference(0))];
        let actual = lex_to_eof(text);
        assert_eq!(expected, actual);

        let text = "     ";
        let expected: Vec<Lexeme> = vec![Lexeme::new(Eof, CharIdx(4), CharDifference(0))];
        let actual = lex_to_eof(text);
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn keywords() {
        {
            let text = "nil";
            let expected: Vec<Lexeme> =
                vec![Lexeme::new(Nil, CharIdx(0), CharDifference(3)), Lexeme::new(Eof, CharIdx(2), CharDifference(0))];
            let test_tokens = lex_to_eof(text);
            assert_eq!(expected, test_tokens);

            let text = "true";
            let expected: Vec<Lexeme> =
                vec![Lexeme::new(True, CharIdx(0), CharDifference(4)), Lexeme::new(Eof, CharIdx(3), CharDifference(0))];
            let test_tokens = lex_to_eof(text);
            assert_eq!(expected, test_tokens);

            let text = "false";
            let expected: Vec<Lexeme> =
                vec![Lexeme::new(False, CharIdx(0), CharDifference(5)), Lexeme::new(Eof, CharIdx(4), CharDifference(0))];
            let test_tokens = lex_to_eof(text);
            assert_eq!(expected, test_tokens);

            let text = "nil\ntrue\tfalse\n";
            let expected: Vec<Lexeme> = vec![
                Lexeme::new(Nil, CharIdx(0), CharDifference(3)),
                Lexeme::new(Newline, CharIdx(3), CharDifference(1)),
                Lexeme::new(True, CharIdx(4), CharDifference(4)),
                Lexeme::new(False, CharIdx(9), CharDifference(5)),
                Lexeme::new(Newline, CharIdx(14), CharDifference(1)),
                Lexeme::new(Eof, CharIdx(14), CharDifference(0)),
            ];
            let test_tokens = lex_to_eof(text);
            assert_eq!(expected, test_tokens);
        }
    }
}
