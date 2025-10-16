//! Attribution: Adapted from `syn` crate:
//! https://docs.rs/syn/latest/src/syn/error.rs.html#225-230

use proc_macro2::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};
use std::fmt;
use std::fmt::{Debug, Display};

pub type Result<T> = std::result::Result<T, Error>;

pub struct Error {
    messages: Vec<ErrorMessage>,
}

pub struct ErrorMessage {
    span: Span,
    message: String,
}

impl Error {
    pub fn new(span: Span, message: String) -> Self {
        Self {
            messages: vec![ErrorMessage {
                span,
                message,
            }]
        }
    }

    pub fn to_compile_error(&self) -> TokenStream {
        self.messages
            .iter()
            .map(ErrorMessage::to_compile_error)
            .collect()
    }
}

impl ErrorMessage {
    fn to_compile_error(&self) -> TokenStream {
        let Self { span, message } = self;

        // ::core::compile_error!($message), but using our span.
        TokenStream::from_iter([
            TokenTree::Punct({
                let mut punct = Punct::new(':', Spacing::Joint);
                punct.set_span(span.clone());
                punct
            }),
            TokenTree::Punct({
                let mut punct = Punct::new(':', Spacing::Alone);
                punct.set_span(span.clone());
                punct
            }),
            TokenTree::Ident(Ident::new("core", span.clone())),
            TokenTree::Punct({
                let mut punct = Punct::new(':', Spacing::Joint);
                punct.set_span(span.clone());
                punct
            }),
            TokenTree::Punct({
                let mut punct = Punct::new(':', Spacing::Alone);
                punct.set_span(span.clone());
                punct
            }),
            TokenTree::Ident(Ident::new("compile_error", span.clone())),
            TokenTree::Punct({
                let mut punct = Punct::new('!', Spacing::Alone);
                punct.set_span(span.clone());
                punct
            }),
            TokenTree::Group({
                let mut group = Group::new(Delimiter::Brace, {
                    TokenStream::from_iter([TokenTree::Literal({
                        let mut string = Literal::string(message);
                        string.set_span(span.clone());
                        string
                    })])
                });
                group.set_span(span.clone());
                group
            }),
        ])
    }

}

impl Debug for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        if self.messages.len() == 1 {
            formatter
                .debug_tuple("Error")
                .field(&self.messages[0])
                .finish()
        } else {
            formatter
                .debug_tuple("Error")
                .field(&self.messages)
                .finish()
        }
    }
}

impl Debug for ErrorMessage {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(&self.message, formatter)
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(&self.messages[0].message)
    }
}
