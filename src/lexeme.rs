use std::ops::{Add, AddAssign, Sub};

/// Starts with line 1.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Line(pub u32);

/// Starts with col 0.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Col(pub u32);

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct CharIdx(pub usize);

impl Add<CharDifference> for CharIdx {
    type Output = Self;

    fn add(self, rhs: CharDifference) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

// impl Sub for CharIdx {
//     type Output = CharDifference;
//
//     fn sub(self, rhs: Self) -> Self::Output {
//         CharDifference(self.0 - rhs.0)
//     }
// }

impl AddAssign<CharDifference> for CharIdx {
    fn add_assign(&mut self, rhs: CharDifference) {
        *self = *self + rhs;
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct CharDifference(pub usize);

/// A lexeme lexed from a text file.
/// The identity of the file will remain implicit until it causes me problems.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Lexeme {
    pub kind: LexemeKind,
    pub start: CharIdx,
    pub len: CharDifference,
}

impl Lexeme {
    pub fn new(
        kind: LexemeKind,
        start: CharIdx,
        len: CharDifference,
    ) -> Self {
        Self {
            kind,
            start,
            len
        }
    }

    /// Get the line and column range of this lexeme in the given program text. O(n).
    pub fn to_line_col_range(self, program_text: &str) -> String {
        todo!()
    }

    /// Get the text spanned by this lexeme. O(n).
    pub fn to_source(self, program_text: &str) -> String {
        program_text.chars().skip(self.start.0).take(self.len.0).collect()
    }
}

pub struct LineColRange {
    pub start_line: Line,
    pub start_col: Col,
    pub end_line: Line,
    pub end_col: Col,
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
