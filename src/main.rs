extern crate rustpiglatin;

use std::io::stdin;
use rustpiglatin as rpl;
fn main() {
    println!("Please enter input to be pigified.");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("failed to get input");
    println!("Your input was {}", rpl::is_lead_vowel(&input));
}
