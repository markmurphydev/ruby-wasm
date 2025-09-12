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
    Nil
}