use proc_macro2::{Span, TokenStream, TokenTree};

pub type ParseInput<'a> = &'a mut ParseStream;
type TokenIter = proc_macro2::token_stream::IntoIter;

#[derive(Debug)]
pub struct ParseStream {
    input: TokenIter,
    peeked: Option<TokenTree>,
    previous_span: Span,
}

impl ParseStream {
    pub fn new(input: TokenStream) -> Self {
        Self {
            input: input.into_iter(),
            peeked: None,
            previous_span: Span::call_site(),
        }
    }

    pub fn next(&mut self) -> Option<TokenTree> {
        match self.peeked.take() {
            Some(v) => Some(v),
            None => self.input.next(),
        }
    }

    pub fn peek(&mut self) -> Option<TokenTree> {
        match &self.peeked {
            Some(tt) => Some(tt.clone()),
            None => {
                let val = self.next();
                self.peeked = val.clone();
                val
            }
        }
    }

    pub fn current_span(&mut self) -> Span {
        match self.peek() {
            Some(tt) => tt.span(),
            None => self.previous_span
        }
    }
}
