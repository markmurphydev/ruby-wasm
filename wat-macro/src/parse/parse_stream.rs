use proc_macro2::{Span, TokenStream, TokenTree};
use std::mem;

pub type ParseInput<'a> = &'a mut ParseStream;
type TokenIter = proc_macro2::token_stream::IntoIter;

#[derive(Debug)]
enum PeekVal {
    None,
    One(TokenTree),
    /// (peek, peek2)
    Two(TokenTree, TokenTree),
    Three(TokenTree, TokenTree, TokenTree),
}

impl Default for PeekVal {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug)]
pub struct ParseStream {
    input: TokenIter,
    // (peek, peek2)
    peeked: PeekVal,
}

impl ParseStream {
    pub fn new(input: TokenStream) -> Self {
        Self {
            input: input.into_iter(),
            peeked: PeekVal::None,
        }
    }

    pub fn next(&mut self) -> Option<TokenTree> {
        match mem::take(&mut self.peeked) {
            PeekVal::None => self.input.next(),
            PeekVal::One(peeked) => Some(peeked),
            PeekVal::Two(first, second) => {
                self.peeked = PeekVal::One(second);
                Some(first)
            }
            PeekVal::Three(first, second, third) => {
                self.peeked = PeekVal::Two(second, third);
                Some(first)
            }
        }
    }

    pub fn peek(&mut self) -> Option<TokenTree> {
        match &self.peeked {
            PeekVal::None => {
                let val = self.input.next();
                self.peeked = match val.clone() {
                    Some(val) => PeekVal::One(val),
                    None => PeekVal::None,
                };
                val
            }
            PeekVal::One(val) | PeekVal::Two(val, _) | PeekVal::Three(val, _, _) => {
                Some(val.clone())
            }
        }
    }

    pub fn peek2(&mut self) -> Option<(TokenTree, TokenTree)> {
        match mem::take(&mut self.peeked) {
            PeekVal::None => {
                let first = self.input.next();
                let second = self.input.next();
                match (first, second) {
                    (Some(first), Some(second)) => {
                        self.peeked = PeekVal::Two(first.clone(), second.clone());
                        Some((first, second))
                    }
                    (Some(first), None) => {
                        self.peeked = PeekVal::One(first);
                        None
                    }
                    (None, None) => {
                        self.peeked = PeekVal::None;
                        None
                    }
                    _ => unreachable!(),
                }
            }
            PeekVal::One(first) => {
                let second = self.input.next();
                match second {
                    Some(second) => {
                        self.peeked = PeekVal::Two(first.clone(), second.clone());
                        Some((first, second))
                    }
                    None => {
                        self.peeked = PeekVal::One(first.clone());
                        None
                    }
                }
            }
            PeekVal::Two(first, second) => {
                self.peeked = PeekVal::Two(first.clone(), second.clone());
                Some((first, second))
            }
            PeekVal::Three(first, second, third) => {
                self.peeked = PeekVal::Three(first.clone(), second.clone(), third);
                Some((first, second))
            }
        }
    }

    pub fn peek3(&mut self) -> Option<(TokenTree, TokenTree, TokenTree)> {
        match mem::take(&mut self.peeked) {
            PeekVal::None => {
                let first = self.input.next();
                let second = self.input.next();
                let third = self.input.next();
                match (first, second, third) {
                    (Some(first), Some(second), Some(third)) => {
                        self.peeked = PeekVal::Three(first.clone(), second.clone(), third.clone());
                        Some((first, second, third))
                    }
                    (Some(first), Some(second), None) => {
                        self.peeked = PeekVal::Two(first, second);
                        None
                    }
                    (Some(first), None, None) => {
                        self.peeked = PeekVal::One(first);
                        None
                    }
                    (None, None, None) => {
                        self.peeked = PeekVal::None;
                        None
                    }
                    _ => unreachable!()
                }
            }
            PeekVal::One(first) => {
                let second = self.input.next();
                let third = self.input.next();
                match (second, third) {
                    (Some(second), Some(third)) => {
                        self.peeked = PeekVal::Three(first.clone(), second.clone(), third.clone());
                        Some((first, second, third))
                    }
                    (Some(second), None) => {
                        self.peeked = PeekVal::Two(first, second);
                        None
                    }
                    (None, None) => {
                        self.peeked = PeekVal::None;
                        None
                    }
                    _ => unreachable!(),
                }
            }
            PeekVal::Two(first, second) => {
                let third = self.input.next();
                match third {
                    Some(third) => {
                        self.peeked = PeekVal::Three(first.clone(), second.clone(), third.clone());
                        Some((first, second, third))
                    }
                    None => {
                        self.peeked = PeekVal::Two(first, second);
                        None
                    }
                }
            }
            PeekVal::Three(first, second, third) => Some((first, second, third)),
        }
    }

    pub fn current_span(&mut self) -> Span {
        match self.peek() {
            Some(tt) => tt.span(),
            None => Span::call_site(),
        }
    }
}
