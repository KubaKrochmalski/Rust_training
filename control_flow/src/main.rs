fn main() {
    let control = 3;

    //if expression
    if control < 5 {
        println!("Control number is lower than 5.");
    } else {
        println!("Control number is NOT lower than 5.");
    }

    //multiple conditions with else if
    let mut number = 15;
    check_divisibility(number);
    number = 12;
    check_divisibility(number);
    number = 17;
    check_divisibility(number);

    //if in a let statement
    let condition = false;
    let x = if condition {
        12
    } else {
        34
    };    
    println!("The value of x is: {}", x);

    //simple loop
    let mut counter = 0;
    loop {
        if counter == 10 {
            println!("===========");
            break;
        }
        println!("I'm looping.");
        counter += 1;
    }

    //while loop
    counter = 0;
    while counter < 10 {
        println!("I'm looping with while condition.");
        counter += 1;
    }
    println!("===========");
    
    //looping through a collection with for
    let array = [3, 4, 7, 13, 19, 23];
    for element in array.iter() {
        println!("Element of an array: {}", element);
    }
    println!("===========");

    //using Range in for loop
    for number in (1..10).rev() {
        println!("==   {}   ==", number);
    }
    println!("== START ==");

    //get the nth Fibonacci number
    let fib = get_fib(5);
    println!("Fibonacci number: {}", fib);
}

fn check_divisibility(x: i32) {
    if x % 5 == 0 {
        println!("The number is divisible by 5.");
    } else if x % 4 == 0 {
        println!("The number is divisible by 4.");
    } else if x % 3 == 0 {
        println!("The number is divisible by 3.");
    } else if x % 2 == 0 {
        println!("The number is divisible by 2.");
    } else {
        println!("The number is not divisible by 5, 4, 3 or 2.");
    }
}

fn get_fib(x: u32) -> u64 {
    let mut fib_number = 1;
    let mut prev_fib = 1;
    let mut tmp;

    if x != 0 && x != 1 {
        for _index in 2..=x {
            tmp = fib_number;
            fib_number += prev_fib;
            prev_fib = tmp;
        }
    }
    fib_number
}
