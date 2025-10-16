use crate::parse::parse_stream::{ParseInput, ParseStream};
use crate::result::{Error, Result};
use proc_macro2::{Delimiter, Ident, Punct, TokenTree, TokenStream};

macro_rules! check_quasi_quote {
    ($input:expr => $exp:expr) => {
        match crate::parse::util::expect_quasi_quote($input) {
            Ok(stream) => Ok(stream),
            Err(_) => $exp
        }
    };
}
pub(crate) use check_quasi_quote;

pub fn expect_quasi_quote(input: ParseInput) -> Result<TokenStream> {
    match peek_quasi_quote(input) {
        Some(stream) => {
            input.next();
            input.next();
            Ok(stream)
        },
        None => Err(error(input, "Expected quasi-quote of the form ,(...)"))
    }
}

pub fn peek_quasi_quote(input: ParseInput) -> Option<TokenStream> {
    match input.peek2() {
        Some((TokenTree::Punct(punct), TokenTree::Group(group))) if punct.as_char() == ',' && group.delimiter() == Delimiter::Parenthesis => {
            Some(group.stream())
        }
        _ => None
    }
}

/// Post: On failure, does not mutate `input`.
pub fn expect_int_literal(input: ParseInput) -> Result<i64> {
    match peek_int_literal(input) {
        Some(n) => {
            input.next();
            Ok(n)
        }
        None => Err(error(input, "Expected int literal.")),
    }
}

/// Post: On failure, does not mutate `input`.
pub fn expect_ident(input: ParseInput) -> Result<Ident> {
    match peek_ident(input) {
        Some(ident) => {
            input.next();
            Ok(ident)
        }
        None => Err(error(input, format!("Expected ident."))),
    }
}

/// Post: On failure, does not mutate `input`.
pub fn expect_ident_named(names: &[&str], input: ParseInput) -> Result<Ident> {
    match peek_ident_named(names, input) {
        Some(ident) => {
            input.next();
            Ok(ident)
        }
        None => Err(error(
            input,
            format!("Expected ident named one of: {:?}", names),
        )),
    }
}

/// Expects a [Delimiter::Parenthesis] group, and returns the inner stream.
/// Post: On failure, does not mutate `input`.
pub fn expect_parens(input: ParseInput) -> Result<ParseStream> {
    match peek_parens(input) {
        Some(stream) => {
            input.next();
            Ok(stream)
        }
        None => Err(error(input, "Expected parens.")),
    }
}

/// Post: On failure, does not mutate `input`.
pub fn expect_dollar(input: ParseInput) -> Result<Punct> {
    match peek_dollar(input) {
        Some(punct) => {
            input.next();
            Ok(punct)
        }
        None => Err(error(input, "Expected `$`")),
    }
}

pub fn expect_sym(input: ParseInput) -> Result<Ident> {
    match peek_sym(input) {
        Some(ident) => {
            input.next();
            input.next();
            Ok(ident)
        }
        None => Err(error(input, "Expected a symbol of the form `$<name>`")),
    }
}

/// Post: On failure, does not mutate `input`.
pub fn expect_open_paren_ident(input: ParseInput) -> Result<(ParseStream, Ident)> {
    match peek_open_paren_ident(input) {
        Some((mut stream, ident)) => {
            // Need to step the outer stream (for the paren), and the inner stream (for the ident).
            input.next();
            stream.next();
            Ok((stream, ident))
        }
        None => Err(error(input, "Expected open paren, then ident.")),
    }
}

/// Post: On failure, does not mutate `input`.
pub fn expect_open_paren_named(names: &[&str], input: ParseInput) -> Result<(ParseStream, Ident)> {
    match peek_open_paren_named(names, input) {
        Some((mut stream, ident)) => {
            // Need to step the outer stream (for the paren), and the inner stream (for the ident).
            input.next();
            stream.next();
            Ok((stream, ident))
        }
        None => Err(error(
            input,
            format!("Expected open paren, then ident named one of: {:?}", names),
        )),
    }
}

/// Post: On failure, does not mutate `input`.
pub fn peek_int_literal(input: ParseInput) -> Option<i64> {
    match input.peek() {
        Some(TokenTree::Literal(lit)) => lit.to_string().parse::<i64>().ok(),
        _ => None,
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

pub fn peek_ident_named(name: &[&str], input: ParseInput) -> Option<Ident> {
    if let Some(ident) = peek_ident(input) {
        if name.contains(&ident.to_string().as_str()) {
            return Some(ident);
        }
    }
    None
}

pub fn peek_dollar(input: ParseInput) -> Option<Punct> {
    match input.peek() {
        Some(TokenTree::Punct(punct)) if punct.as_char() == '$' => Some(punct),
        _ => None,
    }
}

pub fn peek_sym(input: ParseInput) -> Option<Ident> {
    match input.peek2() {
        Some((TokenTree::Punct(punct), TokenTree::Ident(ident))) if punct.as_char() == '$' => {
            Some(ident)
        }
        _ => None,
    }
}

pub fn peek_open_paren_ident(input: ParseInput) -> Option<(ParseStream, Ident)> {
    if let Some(mut body) = peek_parens(input) {
        if let Some(ident) = peek_ident(&mut body) {
            return Some((body, ident));
        }
    }
    None
}

pub fn peek_open_paren_named(names: &[&str], input: ParseInput) -> Option<(ParseStream, Ident)> {
    match peek_open_paren_ident(input) {
        Some((body, ident)) if names.iter().any(|name| ident == name) => {
            return Some((body, ident));
        }
        _ => None,
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
