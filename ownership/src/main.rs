
// 1. Each value in Rust has a variable that’s called its owner.
// 2. There can only be one owner at a time.
// 3. When the owner goes out of scope, the value will be dropped.

fn main() {
    // Both values equal 5, variables on stack
    let x = 5;
    let y = x;
    println!("x: {}", x);
    println!("y: {}", y);

    // s1 being moved to s2
    let s1 = String::from("hello");
    let s2 = s1;

    //println!("{}", s1); - won't work!
    println!("s2 = {}", s2);

    // s3 is a deep copy
    let s3 = s2.clone();
    println!("s2 = {}, s3 = {}", s2, s3);

    // x gets copied, s2 being moved into the function
    makes_copy(x);
    println!("global scope x: {}", x);
    takes_ownership(s2);
    //println!("global scope s2: {}", s2); - won't work!

    // Returning values transfer ownership
    let s1 = gives_ownership();
    println!("returned string: {}", s1);

    let s2 = takes_and_gives_back(s1);
    println!("new s2 = {}", s2);
    //println!("s1 = {}", s1); - won't work!

    // To be able to pass things to a function without losing the ownership
    let (s2, length) = get_length(s2);
    println!("s2 still here = {}, length = {}", s2, length);
}

fn makes_copy(value: i32) {
    println!("copied x: {}", value);
}

fn takes_ownership(text: String) {
    println!("moved s2: {}", text);
}

fn gives_ownership() -> String {
    let new_string = String::from("new hello");
    new_string
}

fn takes_and_gives_back(taken: String) -> String {
    println!("taking ownership and giving it back.");
    taken
}

fn get_length(text: String) -> (String, usize) {
    let length = text.len();

    (text, length)
}
