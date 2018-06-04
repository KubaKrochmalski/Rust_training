struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut kuba = User {
        username: String::from("Jakub Krochmalski"),
        email: String::from("kuba.krochmalski@gmail.com"),
        sign_in_count: 1,
        active: true,
    };

    kuba.username = String::from("Kuba Krochmalski");
    print_user(&kuba);

    let pajko = build_user(String::from("Pajko"),
                           String::from("pajko@gmail.com"));
    print_user(&pajko);

    sign_user(&mut kuba);
    sign_user(&mut kuba);

    // Struct update syntax
    let john = User {
        username: String::from("John_xD"),
        email: String::from("john_123@gmail.com"),
        ..kuba
    };
    print_user(&john);

    // Tuple structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    // They are two different types
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("RGB: {}/{}/{}", black.0, black.1, black.2);
    
    // Destructure a tuple struct
    let Point(x, y, z) = origin;
    println!("Origin: [{}, {}, {}]", x, y, z);
}

// Field init shorthand when variables and fields have the same name
fn build_user(username: String, email: String) -> User {
    User {
        username,         // -> username: username,
        email,            // -> email: email,
        sign_in_count: 1,
        active: false,
    }
}

fn print_user(user: &User) {
    println!("username:      {}", user.username);
    println!("email:         {}", user.email);
    println!("sign in count: {}", user.sign_in_count);
    println!("state:         {}", match user.active {
        false => "inactive",
        true  => "active",
    });
}

fn sign_user(user: &mut User) {
    user.sign_in_count += 1;
}
