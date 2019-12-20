// 29. Bitwise AND
// https://www.hackerrank.com/challenges/30-bitwise-and

use crate::functions;

pub fn main() {
    let mut test_cases: u32 = match String::from(functions::read_line().trim()).parse::<u32>() {
        Ok(n) => n,
        Err(e) => {
            println!("{}", e);
            0
        }
    };

    while test_cases > 0 {
        let input: Vec<u32> = String::from(functions::read_line().trim())
            .split(' ')
            .into_iter()
            .map(|s| s.parse::<u32>())
            .collect::<Result<Vec<u32>, std::num::ParseIntError>>()
            .unwrap();

        let n = input[0];
        let k = input[1];

        let mut max = 0;

        for a in 1..n + 1 {
            for b in a + 1..n + 1 {
                if (a & b) < k {
                    if max < (a & b) {
                        max = a & b;
                    }
                }
            }
        }
        println!("{}", max);
        test_cases -= 1;
    }
}
