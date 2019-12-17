pub fn main() {
    // ----------------------
    // -------- MOVE --------
    // ----------------------
    println!("MOVE EXAMPLE:");
    {
        // the data from s1 is not copied to s2
        let s1 = String::from("hello");
        let s2 = s1;
        // println!("{}", s1); // and it won't work to use s1:
        println!("{}", s2); // but will work with s2

        // it becomes a problem if both strings are pointing to the same allocated memory
        // ^-- this is called: move
    }
    println!();

    // -----------------------
    // -------- CLONE --------
    // -----------------------
    println!("CLONE EXAMPLE:");
    {
        let mut s1 = String::from("hello");
        let mut s2 = s1.clone();
        s1.push_str(" s1");
        s2.push_str(" s2");
        println!("s1 = {}, s2 = {}", s1, s2);
    }
    println!();

    // ----------------------
    // -------- COPY --------
    // ----------------------
    println!("COPY EXAMPLE:");
    {
        // we didn't need to clone() this, because it is an integer.
        // integer's sizes are known at compile time, and are stored on the stack.
        let x = 5;
        let y = x;
        println!("x = {}, y = {}", x, y);
    }
    println!();

    // ----------------------

    println!("OWNERSHIP + FUNCTIONS");
    {
        let s = String::from("hello");
        takes_ownership(s);
        // println!("{}", s); // s no longer valid here...

        let x = 5;
        makes_copy(x);
        println!("{}", x); // because x is <Copy>, we can still use it here
    }
    println!();

    // ----------------------

    println!("WITH RETURN VALUES");
    {
        let s1 = gives_ownership();

        let s2 = String::from("hello");
        let s3 = takes_and_gives_ownership(s2);

        println!("s1 = {}, s2 = not available, s3 = {}", s1, s3);
    }
    println!();
}

fn takes_ownership(some_string: String) {
    println!("some_string = {}", some_string);
} // here "some_string" is dropped

fn makes_copy(some_integer: i32) {
    println!("some_integer = {}", some_integer);
} // here "some_integer" is just out of scope, and not  freed

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_ownership(a_string: String) -> String {
    a_string
}

// now, if we want to pass a value onto a fn,
// and the continue using it afterwards,
// we need to use references (&)
