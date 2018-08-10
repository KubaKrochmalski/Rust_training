enum IPAddV {
    V4,
    V6,
}

struct IPAdd {
    version: IPAddV,
    address: String,
}

// Putting data directly into each enum's variant
enum IPAddress {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quitting..."),
            Message::Move{x, y} => println!("Moving to [{}, {}]", x, y),
            Message::Write(text) => println!("Writing some text: \"{}\"", text),
            Message::ChangeColor(r, g, b) => println!("Changing color to: ({}, {}, {})", r, g, b),
        }
    }
}

fn main() {
    // Creating instance of an enum
    let _version = IPAddV::V4;
    let _version = IPAddV::V6;

    // Enum type variable inside a struct
    let home = IPAdd {
        version: IPAddV::V4,
        address: String::from("127.0.0.1"),
    };

    // Enums with associated data
    let home = IPAddress::V4(127, 0, 0, 1);
    let work = IPAddress::V6(String::from("2001:db8::1428:57ab"));

    // Printing complex enums
    if let IPAddress::V4(n1, n2, n3, n4) = home {
        println!("Home's IP: {}.{}.{}.{}", n1, n2, n3, n4);
    }

    if let IPAddress::V6(address) = work {
        println!("Work's IP: {}", address);
    }

    let msg = Message::Move {x: 3, y: 7};
    msg.call();
    let msg = Message::Write(String::from("HELLO!"));
    msg.call();
    let msg = Message::ChangeColor(137, 64, 112);
    msg.call();

    // Using an Option enum
    let some_number = Some(48);
    let absent_numbet : Option<i32> = None;
}
