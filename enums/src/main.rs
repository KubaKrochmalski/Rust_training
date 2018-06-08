enum IPAddV {
    V4,
    V6,
}

struct IPAdd {
    version: IPAddV,
    address: String,
}

fn main() {
    // Creatin instance of an enum
    let _version = IPAddV::V4;
    let _version = IPAddV::V6;

    let home = IPAdd {
        version: IPAddV::V4,
        address: String::from("127.0.0.1"),
    };
}
