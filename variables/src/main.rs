fn main() {
    //mutability
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    //constants
    const MIN_VALUE: u32 = 0; 
    const MAX_VALUE: u32 = 100_000;
    println!("Allowed range: {} - {}", MIN_VALUE, MAX_VALUE);

    //shadowing
    let y = 5;
    println!("The value of y is: {}", y);
    let y = y + 1;
    println!("The value of y is: {}", y);
    let y = y * 2;
    println!("The value of y is: {}", y);

    //shadowing - type change    
    let spaces = "       ";
    let spaces = spaces.len();
    println!("Number of spaces: {}", spaces);

    //integers
    let my_int = 98_222;
    println!("Decimal: {}", my_int);
    let my_int = 0xff;
    println!("Hex: {}", my_int);
    let my_int = 0o77;
    println!("Octal: {}", my_int);
    let my_int = 0b1111_0000;
    println!("Binary: {}", my_int);
    let my_int = b'A';
    println!("Byte: {}", my_int);

    //floats
    let my_float = 12.34; //f64 by default
    println!("Floating point value: {}", my_float);
    let my_float: f32 = 98.76;
    println!("Floating point value: {}", my_float);

    //numeric operations
    let sum = 5 + 10;
    println!("Sum: {}", sum);
    let difference = 95.5 - 4.3;
    println!("Difference: {}", difference);
    let product = 4 * 30;
    println!("Product: {}", product);
    let quotient = 56.7 / 32.2;
    println!("Quotient: {}", quotient);
    let remainder = 43 % 5;
    println!("Remainder: {}", remainder);
    
    //chars
    let ch = 'a';
    println!("Char: {}", ch);
    let ch = 'A';
    println!("Char: {}", ch);

    //tuples
    let tuple: (i32, f64, String) = (123, 42.24, String::from("Elflord"));
    println!("Tuple: {}, {}, {}", tuple.0, tuple.1, tuple.2);
    let (int, float, string) = tuple;
    println!("Tuple: {}, {}, {}", int, float, string);

    //arrays
    let array = [1, 2, 3, 4, 5];
    let first = array[0];
    let second = array[1];
    println!("Array: {}, {}, {}, {}, {}", first, second, array[2], array[3], array[4]);
}
