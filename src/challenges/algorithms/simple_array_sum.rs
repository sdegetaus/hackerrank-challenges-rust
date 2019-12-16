// Simple Array Sum
// https://www.hackerrank.com/challenges/simple-array-sum

use crate::functions;

pub fn main() {
    let _ = functions::read_line(); // no need to use this value.
    let str_arrays_elements: String = String::from(functions::read_line().trim());
    let elements: Vec<i32> = str_arrays_elements
        .split(' ')
        .into_iter()
        .map(|s| s.parse::<i32>())
        .collect::<Result<Vec<i32>, std::num::ParseIntError>>()
        .unwrap();
    let mut sum = 0;
    for value in elements {
        sum += value;
    }
    println!("{}", sum);
}
