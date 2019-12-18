#![allow(unused_variables)]

// this file also covers the Match pattern and Option<T>

enum SimpleEnum {
    Enum1(String),
    Enum2(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// enums can have methods :O
impl Message {
    fn call(&self) {
        // method body
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

pub fn main() {
    println!();

    // Option <T> Examples:
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // Matching Examples:
    println!("Matching a Option<T>:");
    // invalid! can't add i8 with Option<i8>
    // let sum = x + y;

    // we would have to do it like follows:
    let sum = match y {
        Some(n) => n + x,
        None => 0,
    };
    println!("The sum equals to: {}", sum);

    println!();
    println!(
        "Value of a Quarter in cents: {}",
        value_in_cents(Coin::Quarter)
    );

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        _ => (), // placeholder
    };

    // IF LET
    let some_value = Some(0u8);
    if let Some(3) = some_value {
        println!("three");
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
