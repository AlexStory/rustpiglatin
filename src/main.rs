extern crate rustpiglatin;

use std::io::stdin;
use rustpiglatin as rpl;
fn main() {
    println!("Please enter input:");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("failed to get input");
    println!("{}", rpl::run(input.trim().to_string()));
}
