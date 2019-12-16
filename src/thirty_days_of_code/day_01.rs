// 1. Data Types
// https://www.hackerrank.com/challenges/30-data-types
// NOTE: When I "solved" this challenge it wasn't available for Rust on Hackerrank.

use crate::functions;

pub fn main() {
    let i: i32 = 4;
    let d: f32 = 4.0;
    let mut s: String = String::from("HackerRank ");

    let int_input: i32 = functions::read_line().trim().parse().unwrap();
    let double_input: f32 = functions::read_line().trim().parse().unwrap();
    let string_input: String = String::from(functions::read_line());

    s.push_str(&string_input);

    println!("{}\n{}\n{}", i + int_input, d + double_input, s);
}
