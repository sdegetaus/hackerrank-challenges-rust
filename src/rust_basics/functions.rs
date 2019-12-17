pub fn main() {
    let x = 5;

    // Parameters
    print_number(x); // = 5

    let y = {
        let x = 3;
        x + 1 // no semicolon (to return the value)
    };

    print_number(y); // = 4

    print_number(five()); // = 5

    let z = plus_one(5);
    print_number(z); // = 6
    print_number(plus_one(5)); // = 6
}

fn print_number(x: i32) {
    println!("The value passed is: {}", x);
}

// Function with return type
fn five() -> i32 {
    5
}

// Function with parameter and return
fn plus_one(x: i32) -> i32 {
    x + 1
}
