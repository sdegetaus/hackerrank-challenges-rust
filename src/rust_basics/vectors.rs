#![allow(unused_variables)]

enum Coordinates {
    North(i32),
    East(f64),
    South(String),
    West(u8),
}

pub fn main() {
    // Example 1
    let mut v1 = Vec::new();
    v1.push(1);
    v1.push(2);
    v1.push(3);

    // Example 2
    let v2 = vec![1, 2, 3];

    // Accessing a value
    let third: &i32 = &v2[2];
    match v2.get(2) {
        Some(third) => println!("The third value is {}", third),
        None => println!("No third element"),
    }

    // Iterating
    for i in &v2 {
        println!("{}", i);
    }

    // Iterating over a mutable vector
    for i in &mut v1 {
        *i += 50;
        println!("value: {}", i);
    }

    // Vectors and Enums
    let coordinates = vec![
        Coordinates::North(64),
        Coordinates::East(1.0),
        Coordinates::South(String::from("South")),
        Coordinates::West(8),
    ];
}
