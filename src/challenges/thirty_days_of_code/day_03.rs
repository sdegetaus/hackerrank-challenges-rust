// 3. Intro to Conditional Statements
// https://www.hackerrank.com/challenges/30-conditional-statements

use std::io;

pub fn main() {
    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => match input.trim().parse::<i32>() {
            Ok(n) => {
                println!("{}", result(n));
            }
            Err(e) => println!("Error!\n{}", e),
        },
        Err(e) => println!("Error!\n{}", e),
    }
}

fn result(n: i32) -> String {
    
    let weird = String::from("Weird");
    let not_weird = String::from("Not Weird");

    let is_even: bool = n % 2 == 0;

    if !is_even {
        return weird;
    }

    if is_even && (n >= 2 && n <= 5) {
        return not_weird;
    }

    if is_even && (n >= 6 && n <= 20) {
        return weird;
    }

    if is_even && (n < 20) {
        return not_weird;
    }

    return not_weird;
}