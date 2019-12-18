#![allow(unused_variables)]

pub fn main() {
    let mut s = String::from("hello world");

    // BASIC EXAMPLES
    // let hello = &s[..5]; // same as [0..5]
    // let world = &s[6..]; // [6..11] or [6..len()]
    // println!("{} {}", hello, world);

    let first = first_word(&s[..]);
    println!("{}", first);
    s.clear();

    // same for int arrays:
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
}

fn first_word(s: &str) -> &str {
    // str = string slice
    let bytes = s.as_bytes(); // convert to array of bytes
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            // byte literal
            return &s[..i];
        }
    }
    &s[..]
}
