// 20. Sorting
// https://www.hackerrank.com/challenges/30-sorting

#![allow(unused_variables)]

use crate::functions;

pub fn main() {
    let _ = functions::read_line().trim();
    let input: String = String::from(functions::read_line().trim());

    let mut row = input
        .split(' ')
        .into_iter()
        .map(|s| s.parse::<i32>())
        .collect::<Result<Vec<i32>, std::num::ParseIntError>>()
        .unwrap();

    println!(
        "Array is sorted in {} swaps.",
        swaps_from_bubble_sort(&mut row)
    );
    println!("First Element: {}", row[0]);
    println!("Last Element: {}", row[row.len() - 1]);
}

fn swaps_from_bubble_sort(vec: &mut Vec<i32>) -> i32 {
    let mut number_of_swaps = 0;

    for i in 0..vec.len() {
        for j in 0..vec.len() - 1 {
            if vec[j] > vec[j + 1] {
                vec.swap(j, j + 1);
                number_of_swaps += 1;
            }
        }
    }

    number_of_swaps
}
