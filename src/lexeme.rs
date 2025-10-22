use crate::lexeme::LexemeKind::{Greater, GreaterEqual, Less, LessEqual};
use serde::Serialize;
use std::ops::{Add, AddAssign};

/// Starts with line 1.
#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize)]
pub struct Line(pub u32);

/// Starts with col 0.
#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize)]
pub struct Col(pub u32);

#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize)]
pub struct CharIdx(pub usize);

impl Add<CharDifference> for CharIdx {
    type Output = Self;

    fn add(self, rhs: CharDifference) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign<CharDifference> for CharIdx {
    fn add_assign(&mut self, rhs: CharDifference) {
        *self = *self + rhs;
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize)]
pub struct CharDifference(pub usize);

pub fn text_in_range(text: &str, start_idx: CharIdx, len: CharDifference) -> String {
    text.chars().skip(start_idx.0).take(len.0).collect()
}

/// A lexeme lexed from a text file.
/// The identity of the file will remain implicit until it causes me problems.
#[derive(Debug, Eq, PartialEq, Clone, Serialize)]
pub struct Lexeme {
    pub kind: LexemeKind,
    pub start: CharIdx,
    pub len: CharDifference,
}

impl Lexeme {
    pub fn new(kind: LexemeKind, start: CharIdx, len: CharDifference) -> Self {
        Self { kind, start, len }
    }

    /// Get the text spanned by this lexeme. O(n).
    pub fn to_source(self, program_text: &str) -> String {
        program_text
            .chars()
            .skip(self.start.0)
            .take(self.len.0)
            .collect()
    }

    pub fn is_operator(&self) -> bool {
        use LexemeKind::*;
        match self.kind {
            In | Equal | PipePipe | AmpersandAmpersand | EqualEqual | Greater | GreaterEqual
            | Less | LessEqual | Minus | Plus | Slash | Star | BracketLeft | Dot => true,
            _ => false,
        }
    }

    pub const UNARY_MINUS_BINDING_POWER: u8 = 42;
    /// Gets `(lhs_binding_power, rhs_binding_power)`
    pub fn binding_power(&self) -> (u8, u8) {
        use LexemeKind::*;
        // Copied from Prism `prism.c` `pm_binding_power_t`
        // Higher rhs binding power makes it left-associative.
        const BP_MATCH: (u8, u8) = (12, 13);
        const BP_ASSIGNMENT: (u8, u8) = (18, 18);
        const BP_LOGICAL_OR: (u8, u8) = (24, 25);
        const BP_LOGICAL_AND: (u8, u8) = (26, 27);
        const BP_EQUALITY: (u8, u8) = (28, 29);
        const BP_COMPARISON: (u8, u8) = (30, 31);

        const BP_TERM: (u8, u8) = (38, 39);
        const BP_FACTOR: (u8, u8) = (40, 41);
        const BP_INDEX: (u8, u8) = (48, 49);
        const BP_CALL: (u8, u8) = (50, 50);
        match &self.kind {
            In => BP_MATCH,
            Equal => BP_ASSIGNMENT,
            PipePipe => BP_LOGICAL_OR,
            AmpersandAmpersand => BP_LOGICAL_AND,
            EqualEqual => BP_EQUALITY,
            Greater | GreaterEqual | Less | LessEqual => BP_COMPARISON,
            Minus | Plus => BP_TERM,
            Slash | Star => BP_FACTOR,
            BracketLeft => BP_INDEX,
            Dot => BP_CALL,
            other => panic!("Lexeme of kind {:?} has no binding power", other),
        }
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
#[derive(Debug, Eq, PartialEq, Clone, Serialize)]
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
    IntegerLiteral {
        text: String,
    },
    /// Float of the form `\d[\d_]+\d(\.\d[\d_]+\d)?`
    /// TODO: Recognize float exponents
    FloatLiteral,
    /// String literal surrounded by single quotes.
    /// Allows only the escape characters `\'` and `\\`
    SingleQuoteStringLiteral {
        text: String,
    },

    /// Instance variable of the form `@<IDENTIFIER>`
    InstanceVariable,

    /// Class variable of the form `@@<IDENTIFIER>`
    ClassVariable,

    /// Global variable of the form `$<IDENTIFIER>`
    GlobalVariable {
        text: String,
    },

    /// Constant. Starts with an uppercase letter, then can be any `<IDENTIFIER>` characters
    Constant {
        text: String,
    },
    Identifier {
        text: String,
    },

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
    /// '('
    LeftParen,
    /// ')'
    RightParen,
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
    If,
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
