#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // first parameter is always self, which represents the instance of the struct 
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    // associated functions
    // not methods, often used for constructors returning
    // a new instance of a struct
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = create_rect(10, 15);
    let rect2 = create_rect(10, 9);
    println!("The area of {:#?} is {}.", rect1, rect1.area());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

    // automatic referencing and dereferencing adds *, & or &mut to
    // match the signature of the method. So the following are the same
    /*
    p1.distance(&p2);
    (&p1).distance(&p2);
    */

    // associated functions are called using `::`
    let sqr = Rectangle::square(5);
    println!("Created a square: {:#?}", sqr);
}

fn create_rect(width: u32, height: u32) -> Rectangle {
    let rect = Rectangle { width, height };

    rect
}
