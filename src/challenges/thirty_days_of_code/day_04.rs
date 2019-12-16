// 4. Class vs. Instance
// https://www.hackerrank.com/challenges/30-class-vs-instance

#![allow(non_snake_case)]

use crate::functions;

struct Person {
    age: i32,
}

impl Person {
    fn new(initialAge: i32) -> Person {
        if initialAge < 0 {
            println!("Age is not valid, setting age to 0.");
            return Person { age: 0 };
        }

        return Person { age: initialAge };
    }

    fn amIOld(&self) {
        if self.age < 13 {
            println!("You are young.");
        } else if self.age >= 13 && self.age < 18 {
            println!("You are a teenager.");
        } else {
            println!("You are old.");
        }
    }

    fn yearPasses(&mut self) {
        self.age += 1;
    }
}

pub fn main() {
    let T: i32 = functions::read_line().trim().parse().unwrap();
    for _ in 0..T {
        let age = functions::read_line().trim().parse().unwrap();
        let mut p = Person::new(age);
        p.amIOld();
        for _ in 0..3 {
            p.yearPasses();
        }
        p.amIOld();
        println!("");
    }
}
