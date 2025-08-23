use itertools::PeekNth;
use std::str::Chars;
use crate::token::Token;

pub struct Tokenizer<'text> {
    chars: PeekNth<Chars<'text>>,
    tokens: Vec<Token>,
}

impl <'text> Tokenizer<'text> {
    pub fn new(text: &'text str) -> Self {
        Self {
            chars: itertools::peek_nth(text.chars()),
            tokens: vec![],
        }
    }

    pub fn tokenize(mut self) -> Vec<Token> {

        /// Any number of trailing newlines are semantically identical to zero trailing newlines.
        /// It's more convenient for our parsing to assume zero.
        fn trim_trailing_newlines(tokens: &mut Vec<Token>) {
            let trailing_newline_count = tokens.iter().rev().take_while(|tok| **tok == Token::Newline).count();
            tokens.truncate(tokens.len() - trailing_newline_count);
        }

        loop {
            self.whitespace();
            match self.chars.peek() {
                None => break,
                Some(&c) => match c {
                    '\n' => self.tokens.push(Token::Newline),
                    ';' => self.tokens.push(Token::Semicolon),
                    c if c.is_ascii_alphabetic() => self.keyword_or_identifier(),
                    _ => panic!("Unexpected character: {:#?}", c)
                }
            }
        }

        trim_trailing_newlines(&mut self.tokens);
        self.tokens
    }

    fn keyword_or_identifier(&mut self) {
        // TODO -- Only keyword right now
        assert!(!self.chars.peek().unwrap().is_numeric());
        let mut name = String::new();

        while let Some(c) = self.chars.peek() {
            if c.is_ascii_alphanumeric() {
                name.push(self.chars.next().unwrap());
            } else {
                break;
            }
        }

        match name.as_str() {
            "false" => self.tokens.push(Token::False),
            "true" => self.tokens.push(Token::True),
            "nil" => self.tokens.push(Token::Nil),
            _ => todo!("Not a keyword: {}", name)
        }
    }

    /// Consume non-newline whitespace
    fn whitespace(&mut self) {
        while self.chars.peek().is_some_and(|&c| c.is_whitespace() && c != '\n') {
            self.chars.next();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Token::*;

    #[test]
    pub fn empty() {
        let text = "";
        let expected_tokens: Vec<Token> = vec![];

        let test_tokens = Tokenizer::new(text).tokenize();

        assert_eq!(expected_tokens, test_tokens);
    }

    #[test]
    pub fn keywords() {
        {
            let text = "nil";
            let expected_tokens: Vec<Token> = vec![Nil];

            let test_tokens = Tokenizer::new(text).tokenize();

            assert_eq!(expected_tokens, test_tokens);
        }
        {
            let text = "true";
            let expected_tokens: Vec<Token> = vec![True];

            let test_tokens = Tokenizer::new(text).tokenize();

            assert_eq!(expected_tokens, test_tokens);
        }
        {
            let text = "true false";
            let expected_tokens: Vec<Token> = vec![True, False];

            let test_tokens = Tokenizer::new(text).tokenize();

            assert_eq!(expected_tokens, test_tokens);
        }
    }
}