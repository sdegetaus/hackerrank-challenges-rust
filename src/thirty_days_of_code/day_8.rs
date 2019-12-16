// 8. Dictionaries and Maps
// https://www.hackerrank.com/challenges/30-dictionaries-and-maps

use crate::functions;
use std::collections::HashMap;
use std::convert::TryInto;
use std::io;
use std::io::prelude::*;

pub fn main() {
    let t: i32 = functions::read_line().trim().parse().unwrap();
    let stdin = io::stdin();
    let mut phonebook = HashMap::new();

    for (index, element) in stdin.lock().lines().enumerate() {
        // on empty value, break the std read
        if element.as_ref().unwrap().is_empty() {
            break;
        }
        let input = element.as_ref().unwrap().trim();

        // first: building phonebook
        if index < t.try_into().unwrap() {
            let split_input: Vec<&str> = input.split(" ").collect();
            let name: String = String::from(split_input[0]);
            let phone: String = String::from(split_input[1]);
            phonebook.insert(name, phone);
        }
        // now we are reading from stdin and getting results out from pb
        else {
            if !phonebook.contains_key(input) {
                println!("Not found");
            } else {
                println!("{}={}", input, phonebook[input]);
            }
        }
    }
}
