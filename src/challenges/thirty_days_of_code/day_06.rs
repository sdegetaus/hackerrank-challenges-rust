// 6. Let's Review
// https://www.hackerrank.com/challenges/30-review-loop

use crate::functions;

pub fn main() {
    let T: i32 = functions::read_line().trim().parse().unwrap();
    for i in 0..T {
        let word = String::from(functions::read_line().trim());
        let mut evens = String::new();
        let mut odds = String::new();

        for (index, character) in word.chars().enumerate() {
            if index % 2 == 0 {
                evens.push_str(&character.to_string());
            } else {
                odds.push_str(&character.to_string());
            }
        }

        println!("{} {}", evens, odds);
    }
}
