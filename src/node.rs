#[derive(Debug, Copy, Clone)]
pub struct Program {
    expr: Expr
}

#[derive(Debug, Copy, Clone)]
pub enum Expr {
    Literal(Literal)
}

#[derive(Debug, Copy, Clone)]
pub enum Literal {
    True,
    False,
    Nil
}