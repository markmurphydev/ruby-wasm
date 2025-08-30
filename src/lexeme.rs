// Largely copied from Prism's token list
// https://github.com/ruby/prism
#[derive(Debug, Eq, PartialEq)]
pub enum Lexeme {
    /// The final token in a file.
    Eof,
    /// Newline character outside of other tokens.
    Newline,

    // Punctuation
    Ampersand,
    AmpersandAmpersand,
    AmpersandAmpersandEqual,
    AmpersandEqual,
    AmpersandPeriod,
    /// `` ` ``
    Backtick,
    /// From Prism: "! or !@" ??? I'm not doing that rn.
    Bang,
    /// `!=`
    BangEqual,
    /// `!~`
    BangTilde,
    BraceLeft,
    BraceRight,
    BracketLeft,
    BracketRight,
    /// `[]`
    /// Hmm...
    BracketLeftRight,
    /// `[]=`
    BracketLeftRightEqual,
    /// `^`
    Caret,
    CaretEqual,
    Colon,
    ColonColon,
    /// `,`
    Comma,
    /// `;`
    /// Semicolons can be used as explicit terminators within a single line.
    Semicolon,

    // Keywords
    Alias,
    And,
    Begin,
    BeginUpperCase,
    Break,
    Case,
    Class,
    Do,
    Else,
    Elsif,
    End,
    Ensure,
    False,
    In,
    Nil,
    Not,
    Or,
    Redo,
    Retry,
    Rescue,
    Return,
    SelfKeyword,
    Super,
    True,
    Then,
    Undef,
    Unless,
    Until,
    When,
    While,
    Yield,
    UnderscoreEncoding,
    UnderscoreFile,
    UnderscoreLine,
}
