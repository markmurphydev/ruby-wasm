#[derive(Debug, Eq, PartialEq)]
pub enum Token {
    True,
    False,
    Nil,
    /// In Ruby, newlines act as statement terminators
    /// TODO: Usually? always?
    Newline,
    /// Semicolons can be used as explicit terminators within a single line
    Semicolon,
}