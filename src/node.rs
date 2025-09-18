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
    /// TODO -- Final representation will not be `i64`
    Integer(i64),
    True,
    False,
    Nil,
    If(Box<If>),
    While(Box<While>),
    Until(Box<Until>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct If {
    pub predicate: Expr,
    pub statements: Statements,
    pub subsequent: Subsequent
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct While {
    pub predicate: Expr,
    pub statements: Statements,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Until {
    pub predicate: Expr,
    pub statements: Statements,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Subsequent {
    None,
    Elsif(Box<If>),
    Else(Else)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Else {
    pub statements: Statements,
}