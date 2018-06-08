#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Defining methods
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Associated function
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let rect = Rectangle { width: 50, height: 30 };

    // Printing the rectangle using a Debug trait -> {:?}
    println!("The rectangle: {:?}", rect);
    // Each member in new line -> {:#?}
    println!("The rectangle: {:#?}", rect);

    println!("The area of the rectangle is {} square units.",
             get_area(&rect));
    println!("The area of the rectangle is {} square units.",
             rect.area());

    let rect_small = Rectangle { width: 20, height: 10 };
    let rect_large = Rectangle { width: 70, height: 30 };

    println!("Can rect hold rect_small? Answer: {}", rect.can_hold(&rect_small));
    println!("Can rect hold rect_large? Answer: {}", rect.can_hold(&rect_large));

    let square = Rectangle::square(30);
    println!("The area of the square is {} square units.",
             square.area());
}

fn get_area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
