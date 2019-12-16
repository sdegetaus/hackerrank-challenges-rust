pub fn main() {
    let vec_from: Vec<i32> = vec![2, 3, 4];             // immutable vector

    let mut vec_to: Vec<i32> = Vec::new();              // mutable vector
    vec_to.push(0);
    vec_to.push(1);

    push_all(&vec_from, &mut vec_to);                   // need to lend it (&), and because immutable (mut)

    vec_to.push(5);                                     // still mutable by this fn body

    for v in vec_to.iter() {
        println!("{}", v);
        // vec_to.push(5);                             // not sure why here is not available... perhaps the .iter()?
    }
    
    vec_to.push(6);                                     // I think so! <---------------------------------------^
}

fn push_all(from: &Vec<i32>, to: &mut Vec<i32>) {       // {from} is immutable borrow, but {to} is a mutable borrow/reference
    for elem in from.iter() {
        // from.push(*elem);                            // not possible: is immutable ref/borrow!
        to.push(*elem);                                 // what's the asterisk for?
    }
}
