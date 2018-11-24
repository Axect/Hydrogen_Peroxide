extern crate hydrogen_peroxide;
use hydrogen_peroxide::*;

fn main() {
    let a = Expr::new_number("x");
    println!("{:?}", a);
}