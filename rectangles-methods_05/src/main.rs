#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 'impl': "implementation"
impl Rectangle {
    // method with no explicit arguments
    // <rect_instance>.area()
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // takes and additional argument
    // <rect_instance>.can_hold(<another_rect_instance>)
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // non-method Associatted Function
    // Rectangle::square(<size>)
    // ^ note that this takes the type itselt, not an instance
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let rect2 = Rectangle::square(12);
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
