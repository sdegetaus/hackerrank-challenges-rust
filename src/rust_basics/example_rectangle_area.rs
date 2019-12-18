#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // METHODS
    // when they take an instance of their struct (self)
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // ASSOCIATED FUNCTIONS
    // not methods, as they don't take an instance of their struct (like String::from())
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

pub fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area() // same as &rect.area()
    );

    let rect2 = Rectangle {
        width: 20,
        height: 30,
    };

    println!("rect = {:?}", rect1); // {:?} or {:#?} tell println!() we want the Debug trait

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

    println!("Associative Method (square): {:#?}", &Rectangle::square(3));
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
