// 17. More Exceptions
// https://www.hackerrank.com/challenges/30-more-exceptions
// NOTE: When I "solved" this challenge it wasn't available for Rust on Hackerrank.

use crate::functions;

pub fn main() {
    let mut test_cases: u32 = String::from(functions::read_line().trim())
        .parse::<u32>()
        .unwrap();
    while test_cases > 1 {
        let mut input: Vec<u32> = vec![];
        match String::from(functions::read_line().trim())
            .split(' ')
            .into_iter()
            .map(|s| s.parse::<u32>())
            .collect()
        {
            Ok(v) => {
                input = v;
            }
            Err(_) => {
                println!("n and p should be non-negative");
                test_cases -= 1;
                continue;
            }
        };
        println!("{}", myCalculator::power(input[0], input[1]));
        test_cases -= 1;
    }
}

struct myCalculator;
impl myCalculator {
    fn power(n: u32, p: u32) -> u32 {
        n.pow(p)
    }
}
