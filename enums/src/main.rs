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
}
