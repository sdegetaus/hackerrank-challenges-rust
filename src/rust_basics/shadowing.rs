pub fn main() {
    let x = 2;              // x = 2
    let x = x + 1;          // x = 3
    let x = x * 2;          // x = 6
    println!("x = {}", x);  // x = 6

    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces.len() = {}", spaces);

    // Not Allowed:
    // let mut spaces = "";
    // spaces = spaces.len();
}
