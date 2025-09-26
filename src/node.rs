use serde::Serialize;

/// Root of the AST
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Program {
    pub statements: Statements
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Statements {
    pub body: Vec<Expr>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum Expr {
    /// TODO -- Final representation will not be `i64`
    Integer(i64),
    True,
    False,
    Nil,
    GlobalVariableWrite(Box<GlobalVariableWrite>),
    GlobalVariableRead(Box<GlobalVariableRead>),
    If(Box<If>),
    While(Box<While>),
    Until(Box<Until>),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GlobalVariableWrite {
    pub name: String,
    pub expr: Expr,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GlobalVariableRead {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct If {
    pub predicate: Expr,
    pub statements: Statements,
    pub subsequent: Subsequent
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct While {
    pub predicate: Expr,
    pub statements: Statements,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Until {
    pub predicate: Expr,
    pub statements: Statements,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum Subsequent {
    None,
    Elsif(Box<If>),
    Else(Else)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Else {
    pub statements: Statements,
}