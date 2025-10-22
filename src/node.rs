use serde::Serialize;

/// Root of the AST
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Program {
    pub statements: Statements,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Statements {
    pub body: Vec<Expr>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum Expr {
    /// TODO -- Final representation will not be `i64`
    Integer(i64),
    SingleQuoteString(String),
    True,
    False,
    Nil,
    GlobalVariableWrite(Box<GlobalVariableWrite>),
    GlobalVariableRead(Box<GlobalVariableRead>),
    ConstantWrite(Box<ConstantWrite>),
    ConstantRead(Box<ConstantRead>),
    If(Box<If>),
    While(Box<While>),
    Until(Box<Until>),
    Call(Box<Call>),
    And(Box<And>),
    Or(Box<Or>),
    Array(Box<Array>),
    Def(Box<Def>),
    LocalVariableRead(Box<LocalVariableRead>),
    LocalVariableWrite(Box<LocalVariableWrite>),
}

/// Method definition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LocalVariableWrite {
    pub name: String,
    pub val: Expr,
}

/// Method definition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct LocalVariableRead {
    pub name: String,
}

/// Method definition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Def {
    pub name: String,
    pub params: Vec<RequiredParam>,
    pub body: Statements,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RequiredParam {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Array {
    pub vals: Vec<Expr>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct And {
    pub lhs: Expr,
    pub rhs: Expr,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Or {
    pub lhs: Expr,
    pub rhs: Expr,
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
pub struct ConstantWrite {
    pub name: String,
    pub expr: Expr,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ConstantRead {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct If {
    pub predicate: Expr,
    pub statements: Statements,
    pub subsequent: Subsequent,
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
pub struct Call {
    pub receiver: Option<Expr>,
    pub name: String,
    pub args: Vec<Expr>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub enum Subsequent {
    None,
    Elsif(Box<If>),
    Else(Else),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Else {
    pub statements: Statements,
}
