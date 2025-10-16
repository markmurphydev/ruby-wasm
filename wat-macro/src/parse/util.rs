use proc_macro2::{Delimiter, Ident, TokenTree};
use crate::parse::parse_stream::{ParseInput, ParseStream};
use crate::result::Error;

/// `(input: ParseInput) -> Result<i64>`
macro_rules! expect_int_literal {
    ($input:expr) => {
        expect_int_literal_fn(file!(), line!(), $input)
    };
}

/// `(input: ParseInput) -> Result<ParseStream>`
macro_rules! expect_ident {
    ($input:expr) => {
        expect_ident_fn(file!(), line!(), $input)
    };
}

/// `(name: &str, input: ParseInput) -> Result<i64>`
macro_rules! expect_ident_named {
    ($name:expr, $input:expr) => {
        expect_ident_named_fn(file!(), line!(), $name, $input)
    };
}

/// `(input: ParseInput) -> Result<ParseStream>`
macro_rules! expect_parens {
    ($input:expr) => {
        expect_parens_fn(file!(), line!(), $input)
    };
}

/// `(input: ParseInput) -> Result<()>`
macro_rules! expect_dollar {
    ($input:expr) => {
        expect_dollar_fn(file!(), line!(), $input)
    };
}

pub fn expect_int_literal_fn(file: &str, line: u32, input: ParseInput) -> crate::result::Result<i64> {
    match input.next() {
        Some(TokenTree::Literal(lit)) => lit
            .to_string()
            .parse::<i64>()
            .map_err(|err| error(input, format!("{}", err))),
        _ => Err(error(
            input,
            format!("{}:{} -- Expected int literal.", file, line),
        )),
    }
}

pub fn expect_ident_fn(file: &str, line: u32, input: ParseInput) -> crate::result::Result<Ident> {
    match input.next() {
        Some(TokenTree::Ident(ident)) => Ok(ident),
        _ => Err(error(
            input,
            format!("{}:{} -- Expected ident.", file, line),
        )),
    }
}

pub fn expect_ident_named_fn(file: &str, line: u32, name: &str, input: ParseInput) -> crate::result::Result<Ident> {
    match input.next() {
        Some(TokenTree::Ident(ident)) if ident == name => Ok(ident),
        _ => Err(error(
            input,
            format!("{}:{} -- Expected ident named {}.", file, line, name),
        )),
    }
}

/// Expects a [Delimiter::Parenthesis] group, and returns the inner stream.
pub fn expect_parens_fn(file: &str, line: u32, input: ParseInput) -> crate::result::Result<ParseStream> {
    match input.next() {
        Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Parenthesis => {
            Ok(ParseStream::new(group.stream()))
        }
        _ => Err(error(
            input,
            format!("{}:{} -- Expected parens.", file, line),
        )),
    }
}

pub fn peek_parens(input: ParseInput) -> Option<ParseStream> {
    match input.peek() {
        Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Parenthesis => {
            Some(ParseStream::new(group.stream()))
        }
        _ => None,
    }
}

pub fn peek_ident(input: ParseInput) -> Option<Ident> {
    match input.peek() {
        Some(TokenTree::Ident(ident)) => Some(ident),
        _ => None,
    }
}

pub fn peek_ident_named(name: &str, input: ParseInput) -> bool {
    if let Some(ident) = peek_ident(input) {
        if ident == name {
            return true;
        }
    }
    false
}

pub fn peek_dollar(input: ParseInput) -> bool {
    match input.peek() {
        Some(TokenTree::Punct(punct)) => punct.as_char() == '$',
        _ => false,
    }
}

pub fn peek_open_paren_named(names: &[&str], input: ParseInput) -> bool {
    if let Some(mut body) = peek_parens(input) {
        if let Some(ident) = peek_ident(&mut body) {
            return names.iter().any(|name| ident == name)
        }
    }
    false
}

pub fn expect_dollar_fn(file: &str, line: u32, input: ParseInput) -> crate::result::Result<()> {
    match input.next() {
        Some(TokenTree::Punct(punct)) if punct.as_char() == '$' => Ok(()),
        _ => Err(error(input, format!("{}:{} -- Expected `$`.", file, line))),
    }
}

/// Post: Does not mutate `input` (except for peek)
pub fn error<M>(input: ParseInput, message: M) -> Error
where
    M: Clone + Into<String>,
{
    let span = input.current_span();
    Error::new(span, message.into())
}

pub(crate) use expect_int_literal;
pub(crate) use expect_ident;
pub(crate) use expect_ident_named;
pub(crate) use expect_parens;
pub(crate) use expect_dollar;
