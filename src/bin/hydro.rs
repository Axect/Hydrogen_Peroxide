extern crate hydrogen_peroxide;
use hydrogen_peroxide::*;

fn main() {
    let a = Symbol::new_number("x");
    println!("{:?}", a);

    let b = Symbol::new_number("y");

    let c = Symbol::new_number("z");

    println!("{:?}", c * (a + b));
}