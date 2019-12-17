pub fn main() {
    a_loop();
    for_loop();
}

fn a_loop() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("Loop that returns a value: {}", result);
}

fn for_loop() {
    let a = [10, 20, 30, 40, 50];

    // Forward Iterate
    for element in a.iter() {
        println!("For loop value: {}", element);
    }

    // Reverse Iterate
    for element in a.iter().rev() {
        println!("[Reversed] For loop value: {}", element);
    }
}
