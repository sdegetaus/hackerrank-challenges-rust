// 7. Arrays
// https://www.hackerrank.com/challenges/30-arrays/problem

use crate::functions;

pub fn main() {
    let _: String = functions::read_line(); // first line not needed (size of array)
    let str_numbers: String = String::from(functions::read_line().trim());

    let mut numbers = str_numbers.split(" ").collect::<Vec<&str>>();
    numbers.reverse();

    println!("{}", numbers.join(" "));
}
