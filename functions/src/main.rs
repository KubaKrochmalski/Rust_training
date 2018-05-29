fn main() {
    println!("Main(): Hello!");

    function(42);
    another_function(123, String::from("duck"));

    //An expression (no semi-colon)
    let x  = {
        let y = 4;
        y + 3
    };
    println!("The value of x is: {}", x);

    //Return from function
    let x = get_five();
    println!("The new x is: {}", x);

    let x = plus_one(x);
    println!("The incremented x is: {}", x);
}

fn function(val: i32) {
    println!("Function's value: {}", val);
}

fn another_function(val: i32, text: String) {
    println!("Another function's value: {}", val);
    println!("Another function's text: \"{}\"", text);
}

fn get_five() -> i32 {
    5
}

fn plus_one(val: i32) -> i32 {
    val + 1
}
