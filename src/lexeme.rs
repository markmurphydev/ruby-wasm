/// Starts with line 1.
pub type Line = u64;

/// Starts with col 0.
pub type Col = u64;

/// A lexeme lexed from a text file.
/// The identity of the file will remain implicit until it causes me problems.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Lexeme {
    pub kind: LexemeKind,
    /// Starts with line 1.
    pub start_line: Line,
    /// Starts with col 0.
    pub start_col: Col,
    /// Starts with line 1. Inclusive.
    pub end_line: Line,
    /// Starts with col 0. Exclusive.
    pub end_col: Col,
}

impl Lexeme {
    pub fn new(
        kind: LexemeKind,
        start_line: Line,
        start_col: Col,
        end_line: Line,
        end_col: Col,
    ) -> Self {
        Self {
            kind,
            start_line,
            start_col,
            end_line,
            end_col,
        }
    }
}

/// Largely copied from Prism's token list
/// https://github.com/ruby/prism (MIT license)
/// These are all "dumb" enum values -- they don't store their values (for eg. integers)
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum LexemeKind {
    /// The final token in a file.
    Eof,
    /// Newline character outside of other tokens.
    Newline,
    /// Single-line comment of the form `#[^\n]*`
    InlineComment,
    /// Character literal of the form `?<CHAR>`
    CharacterLiteral,
    /// Integer of the form `\d[\d_]+\d`
    /// TODO: Recognize 0x, 0b, ...
    IntegerLiteral,
    /// Float of the form `\d[\d_]+\d(\.\d[\d_]+\d)?`
    /// TODO: Recognize float exponents
    FloatLiteral,
    /// String literal surrounded by single quotes.
    /// Allows only the escape characters `\'` and `\\`
    SingleQuoteStringLiteral,

    /// Instance variable of the form `@<IDENTIFIER>`
    InstanceVariable,

    /// Class variable of the form `@@<IDENTIFIER>`
    ClassVariable,

    /// Constant. Starts with an uppercase letter, then can be any `<IDENTIFIER>` characters
    Constant,
    Identifier,

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
    /// '{'
    BraceLeft,
    /// '}'
    BraceRight,
    /// `[`
    BracketLeft,
    /// `[]`
    BracketLeftRight,
    /// `[]=`
    BracketLeftRightEqual,
    /// `]`
    BracketRight,
    /// `^`
    Caret,
    /// `^=`
    CaretEqual,
    /// `:`
    Colon,
    /// `::`
    ColonColon,
    /// `,`
    Comma,
    /// `.`
    Dot,
    /// `..`
    DotDot,
    /// `...`
    DotDotDot,
    /// `=`
    Equal,
    /// `==`
    EqualEqual,
    /// `===`
    EqualEqualEqual,
    /// `=>`
    EqualGreater,
    /// `=~`
    EqualTilde,
    /// `>`
    Greater,
    /// `>=`
    GreaterEqual,
    /// `>>`
    GreaterGreater,
    /// `>>=`
    GreaterGreaterEqual,
    /// '<'
    Less,
    /// '<='
    LessEqual,
    /// '<=>'
    LessEqualGreater,
    /// '<<'
    LessLess,
    /// '<<='
    LessLessEqual,
    /// '-'
    Minus,
    /// '-@'
    MinusAt,
    /// '-='
    MinusEqual,
    /// '->'
    MinusGreater,
    /// '%'
    Percent,
    /// '%='
    PercentEqual,
    /// '%i'
    PercentLowerI,
    /// '%w'
    PercentLowerW,
    /// '%x'
    PercentLowerX,
    /// '%I'
    PercentUpperI,
    /// '%W'
    PercentUpperW,
    /// '|'
    Pipe,
    /// '|='
    PipeEqual,
    /// '||'
    PipePipe,
    /// '||='
    PipePipeEqual,
    /// '+'
    Plus,
    /// '+@'
    PlusAt,
    /// '+='
    PlusEqual,
    /// '?'
    Question,
    /// '/'
    Slash,
    /// '/='
    SlashEqual,
    /// '*'
    Star,
    /// '*='
    StarEqual,
    /// '**'
    StarStar,
    /// '**='
    StarStarEqual,
    /// `~` or `~@`
    /// TODO: ???
    TildeOrTildeAt,
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
    Def,
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
    /// "__ENCODING__"
    UnderscoreEncoding,
    /// "__FILE__"
    UnderscoreFile,
    /// "__LINE__"
    UnderscoreLine,
}
