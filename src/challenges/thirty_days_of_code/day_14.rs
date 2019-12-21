// 14. Scope
// https://www.hackerrank.com/challenges/30-scope
// NOTE: When I "solved" this challenge it wasn't available for Rust on Hackerrank.

use crate::functions;

pub fn main() {
    let _ = String::from(functions::read_line().trim());
    let a: Vec<i32> = String::from(functions::read_line().trim())
        .split(' ')
        .into_iter()
        .map(|s| s.parse::<i32>())
        .collect::<Result<Vec<i32>, std::num::ParseIntError>>()
        .unwrap();
    let mut d = Difference {
        elements: a,
        maximum_difference: 0,
    };
    println!("{}", d.compute_difference());
}

struct Difference {
    elements: Vec<i32>,
    maximum_difference: i32,
}

impl Difference {
    fn compute_difference(&mut self) -> i32 {
        for i in 0..self.elements.len() {
            for j in i + 1..self.elements.len() {
                let difference = (self.elements[i] - self.elements[j]).abs();
                if difference > self.maximum_difference {
                    self.maximum_difference = difference;
                }
            }
        }
        self.maximum_difference
    }
}
