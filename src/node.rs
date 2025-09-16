/// Root of the AST
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    pub statements: Statements
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Statements {
    pub body: Vec<Expr>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    /// TODO -- Final representation might not be `u64`
    Integer(i64),
    True,
    False,
    Nil,
    If(Box<If>),
    Else(Box<Else>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct If {
    predicate: Expr,
    statements: Statements,
    subsequent: Subsequent
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// "elsif" -> `If { subsequent: Else }`
enum Subsequent {
    None,
    If(Box<If>),
    Else(Else)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Else {
    statements: Statements,
}