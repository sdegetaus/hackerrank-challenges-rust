// 5. Loops
// https://www.hackerrank.com/challenges/30-loops

use crate::functions;

pub fn main() {
    let num: i32 = functions::read_line().trim().parse().unwrap();
    for i in 1..11 {
        println!("{} x {} = {}", num, i, num * i);
    }
}
