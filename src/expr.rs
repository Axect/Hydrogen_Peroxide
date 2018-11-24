pub use self::Expr::*;
use std::ops;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub struct Symbol {
    name: String,
    expr: Expr,
}

impl Symbol {
    pub fn new(name: String, expr: Expr) -> Symbol {
        Symbol { name: name, expr: expr }
    }

    pub fn new_number(name: &str) -> Symbol {
        Symbol { name: name.to_string(), expr: Number }
    }
}

impl ops::Add<Symbol> for Symbol {
    type Output = Symbol;
    fn add(self, other: Symbol) -> Symbol {
        let name_self = self.name;
        let name_other = other.name;
        let name = format!("({} + {})", name_self, name_other);

        let expr = Add(Box::new(self.expr), Box::new(other.expr));

        Symbol { name: name, expr: expr }
    }
}

impl ops::Mul<Symbol> for Symbol {
    type Output = Symbol;
    fn mul(self, other: Symbol) -> Symbol {
        let name = format!("({} * {})", self.name, other.name);
        let expr = Mul(Box::new(self.expr), Box::new(other.expr));
        Symbol::new(name, expr)
    }
}