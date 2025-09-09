/// Root of the AST
#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Statements
}

#[derive(Debug, Clone)]
pub struct Statements {
    pub body: Vec<Expr>,
}

#[derive(Debug, Clone)]
pub enum Expr {
    True,
    False,
    Nil
}