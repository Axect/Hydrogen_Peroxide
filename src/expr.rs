pub use self::Expr::*;
use std::ops::{Add};

#[derive(Debug)]
pub enum Expr {
    Number,
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

#[derive(Debug)]
pub struct Symbol {
    name: String,
    expr: Expr,
}

impl Symbol {
    pub fn new_number(name: &str) -> Symbol {
        Symbol { name: name.to_string(), expr: Number }
    }
}

impl Add<Symbol> for Symbol {
    type Output = Symbol;
    fn add(self, other: Symbol) -> Symbol {
        let name_self = self.name;
        let name_other = other.name;
        let name = format!("{} + {}", name_self, name_other);

        let expr = match (self.expr, other.expr) {
            (Number, Number) => Add(Box::new(Number), Box::new(Number)),
            (Number, Add(x, y)) => Add(Box::new(Number), Box::new(Add(x, y))),
            _ => Number,
        };

        Symbol { name: name, expr: expr }
    }
}