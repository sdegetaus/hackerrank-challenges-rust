pub fn main() {
    println!("REFERENCES + BORROWING");
    {
        // having references as function parameters is called BORROWING
        let s1 = String::from("hello");
        let len = calculate_length(&s1); // references allow to refere to a value, without taking its ownership
        println!("s1.len() = {}", len);
        // * is the opposite: dereferencing)
    }
    println!();

    // ----------------------

    println!("MUTABLE REFERENCES");
    {
        let mut s = String::from("hello");
        mutable_reference(&mut s);
        println!("s = {}", s);
        // we can only have one mutable reference of a variable: {
        //     let mut s = String::from("hello");
        //     let r1 = &mut s;
        //     let r2 = &mut s;
        //     println!("{}, {}", r1, r2);
        // }

        // we can only have either ONE mutable reference or ONE immutable (not both)
        // but we can have MULTIPLE immutable references: {
        //     let mut s = String::from("hello");
        //     let r1 = &s;
        //     let r2 = &s;
        //     let r3 = &mut s;
        //     println!("{}, {}, {}", r1, r2, r3);
        // }
    }
    println!();
}

fn calculate_length(s: &String) -> usize {
    // s.push_str("example"); // not valid: references are not mutable by default (like variables)
    s.len()
} // as it doesn't have ownership of the value passed, nothing happens

fn mutable_reference(s: &mut String) {
    s.push_str(", world");
}
