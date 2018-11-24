pub use self::Expr::*;

#[derive(Debug)]
pub enum Expr {
    Number(String),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Poly(Box<Expr>),
    Sin(Box<Expr>),
    Cos(Box<Expr>),
    Tan(Box<Expr>),
    Pow(Box<Expr>, Box<Expr>),
    Exp(Box<Expr>),
    Log(Box<Expr>),
}

impl Expr {
    pub fn new_number(variable: &str) -> Expr {
        Number(variable.to_string())
    }
}