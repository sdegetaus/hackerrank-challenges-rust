// 10. Binary Numbers
// https://www.hackerrank.com/challenges/30-binary-numbers

use crate::functions;

pub fn main() {
    let input: i32 = functions::read_line().trim().parse().unwrap();
    let binary: String = format!("{:b}", input);

    let mut curr_seq = 0;
    let mut max_seq = 0;

    for c in binary.chars() {
        if c == '1' {
            curr_seq += 1;
        } else {
            curr_seq = 0;
        }

        if max_seq < curr_seq {
            max_seq = curr_seq;
        }
    }
    println!("{}", max_seq);
}
