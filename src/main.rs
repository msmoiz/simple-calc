#![allow(clippy::expect_fun_call)]

use crate::calculator::Calculator;

mod calculator;

fn main() {
    let result = Calculator::evaluate("1 + 2 + 3 + 4 + 5 * 14 / 7");
    println!("Result is {}", result.unwrap());
}
