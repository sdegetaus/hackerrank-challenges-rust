pub fn main() {
    lender();
}

fn lender() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    use_it(&vec);               // need to borrow here (&)
    vec.push(3);                // here we can alter it, as we only borrowed it
}

fn use_it(vec: &Vec<i32>) {     // vec here is a shared reference
    // vec.push(3);             // not allowed, it is NOT mutable
    println!("{}", vec[0]);     // but we can read!
    // the lend has finished...
}