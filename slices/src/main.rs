fn main() {
    // Attempt to get the first word of a sentence
    // without using slices
    let mut s1 = String::from("Andrew has a cow.");
    let mut s2 = String::from("Andrew_has_a_cow.");
    let space_pos1 = get_space_pos(&s1);
    let space_pos2 = get_space_pos(&s2);

    println!("Length of the first word in s1: {}", space_pos1);
    println!("Length of the first word in s2: {}", space_pos2);

    s1.clear();
    s2.clear();
    // Variables holding space positions have no sense now
    // This solution is error prone

    // Using string slices
    let s = String::from("The cow is happy.");
    let animal = &s[4..7];
    let state = &s[11..16];

    println!("Animal: {}", animal);
    println!("State: {}", state);

    // Attempt to get the first word of a sentence
    // with string slices
    let mut s1 = String::from("Andrew has a cow.");
    let word = get_first_word(&s1);
    println!("First word: {}", word);
    //s1.clear(); - won't work! Trying a mutable borrow when immutable borrow still active

    // get_first_word() improved -> first_word() - works also on string literals
    // string literals are actually string slices
    let word = first_word(&s1[..]);
    println!("First word: {}", word);

    let literal = "Zombies eat brains.";
    let word = first_word(literal);
    println!("First word: {}", word);

    // Slice of an i32 values array -> type: &[i32]
    let array = [15, 78, 1, 2, 3, 23, 41, 16, 4];
    let slice = &array[2..5];
    for x in slice.iter() {
        print!("{} ", x);
    }
}

fn get_space_pos(text: &String) -> usize {
    let bytes = text.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    text.len()
}

fn get_first_word(text: &String) -> &str {
    let bytes = text.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &text[..i];
        }
    }
    &text[..]
}

fn first_word(text: &str) -> &str {
    let bytes = text.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &text[..i];
        }
    }
    text
}
