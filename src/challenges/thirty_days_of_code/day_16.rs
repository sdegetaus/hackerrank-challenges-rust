// 16. Exceptions - String to Integer
// https://www.hackerrank.com/challenges/30-exceptions-string-to-integer
// NOTE: When I "solved" this challenge it wasn't available for Rust on Hackerrank.

use crate::functions;

pub fn main() {
    match functions::read_line().trim().parse::<i32>() {
        Ok(n) => {
            println!("{}", n);
        }
        Err(_) => {
            println!("Bad String");
        }
    };
}
