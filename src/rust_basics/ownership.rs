pub fn main() {
    give();
}

fn give() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    take(vec);
    //vec.push(3);              // no longer accesible, it has borrowed it to take()
}

fn take(vec: Vec<i32>) {
    // vec.push(3);             // not allowed, vec is not mutable 
    println!("{}", vec[0]);     // can't write, but it can read
    // here vec is now dead!
}
