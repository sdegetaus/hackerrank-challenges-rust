// 26. Nested Logic
// https://www.hackerrank.com/challenges/30-nested-logic

use crate::functions;

struct Date {
    day: u32,
    month: u32,
    year: u32,
}

pub fn main() {
    // returned
    let input_returned = String::from(functions::read_line().trim())
        .split(' ')
        .into_iter()
        .map(|s| s.parse::<u32>())
        .collect::<Result<Vec<u32>, std::num::ParseIntError>>()
        .unwrap();

    // expected
    let input_expected = String::from(functions::read_line().trim())
        .split(' ')
        .into_iter()
        .map(|s| s.parse::<u32>())
        .collect::<Result<Vec<u32>, std::num::ParseIntError>>()
        .unwrap();

    let date_returned = Date {
        day: input_returned[0],
        month: input_returned[1],
        year: input_returned[2],
    };

    let date_expected = Date {
        day: input_expected[0],
        month: input_expected[1],
        year: input_expected[2],
    };

    println!("{}", get_fine(date_returned, date_expected));
}

fn get_fine(returned: Date, expected: Date) -> u32 {
    if returned.year < expected.year {
        return 0;
    }
    if returned.year > expected.year {
        return 10000;
    }
    if returned.month > expected.month {
        return (returned.month - expected.month) * 500;
    }
    if returned.day > expected.day {
        return (returned.day - expected.day) * 15;
    }
    0
}
