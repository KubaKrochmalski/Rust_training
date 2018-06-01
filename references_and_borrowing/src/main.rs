
// At any given time, you can have either (but not both of):
// - one mutable reference;
// - any number of immutable references.
// References must always be valid.

fn main() {
    // Passing a reference to an object.
    // This is called "borrowing".
    let s1 = String::from("I saw a wild duck");
    let len = get_length(&s1);
    println!("The length of \"{}\" is {}", s1, len);

    let mut s2 = s1.clone();
    change(&mut s2);
    println!("s2 after change = \"{}\"", s2);
}

// This function cannot change a borrowed value
fn get_length(text: &String) -> usize {
    text.len()
}

// Borrowed value can be changed via mutable reference
fn change(text: &mut String) {
    text.push_str(" in the woods");
}