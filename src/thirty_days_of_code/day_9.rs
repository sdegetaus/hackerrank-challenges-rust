// 9. Recursion
// https://www.hackerrank.com/challenges/30-recursion

use crate::functions;

pub fn main() {
    let t: i32 = functions::read_line().trim().parse().unwrap();
    println!("{}", factorial(t));
}

pub fn factorial(n: i32) -> i32 {
    if n <= 1 {
        return 1;
    }
    return n * factorial(n - 1);
}
