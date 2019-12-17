pub fn main() {
    let x: u32 = "42".parse().expect("Not a number!");
    println!("x = {}", x);

    // Integer Literals
    let decimal: i32 = 100_000;
    let hex: i32 = 0xff;
    let octal: i32 = 0o77;
    let binary: i32 = 0b1111_0000;
    println!("{}, {}, {}, {}", decimal, hex, octal, binary);

    // Arrays
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [1i32; 5];
}
