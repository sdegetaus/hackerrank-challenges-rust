// 11. 2D Arrays
// https://www.hackerrank.com/challenges/30-2d-arrays

use crate::functions;

const DIMENSION: usize = 6;

pub fn main() {
    let mut arr = [[0i32; DIMENSION]; DIMENSION];

    for i in 0..DIMENSION {
        let input: String = String::from(functions::read_line().trim());

        let row = input
            .split(' ')
            .into_iter()
            .map(|s| s.parse::<i32>())
            .collect::<Result<Vec<i32>, std::num::ParseIntError>>()
            .unwrap();

        let mut j = 0;
        for value in row {
            arr[i][j] = value;
            j += 1;
        }
    }

    let mut max_sum: i32 = 0;

    for i in 0..(DIMENSION - 2) {
        for j in 0..(DIMENSION - 2) {
            let current_sum: i32 = arr[i][j]
                + arr[i][j + 1]
                + arr[i][j + 2]
                + arr[i + 1][j + 1]
                + arr[i + 2][j]
                + arr[i + 2][j + 1]
                + arr[i + 2][j + 2];

            if i == 0 && j == 0 {
                max_sum = current_sum;
                continue;
            }
            if current_sum > max_sum {
                max_sum = current_sum;
            }
        }
    }

    println!("{}", max_sum);
}
