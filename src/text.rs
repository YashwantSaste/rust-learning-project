//! String and slice examples.
//!
//! The helper functions here show the difference between returning an owned
//! string and borrowing a slice from existing string data.

/// Runs the string and slice examples.
pub fn run() {
    println!("== Strings and Slices ==");

    let mut string1 = String::from("Hello");
    string1.push_str(" World");
    println!("String after push_str: {}", string1);

    let first_word = get_first_word_owned(&string1);
    println!("First word: {}", first_word);

    let string2 = String::from("Hello Rustaceans");
    let first_word_slice = get_first_word_slice(&string2);
    println!("First word using slice: {}", first_word_slice);

    let first_type = String::from("Hello");
    let second_type = "Hello";
    let third_type = first_type.as_str();
    println!(
        "String forms: owned={}, slice={}, borrowed={}",
        first_type, second_type, third_type
    );

    let array = [1, 2, 3, 4, 5];
    let slice = &array[1..4];
    println!("Array slice: {:?}", slice);
}

/// Returns the first word as an owned `String`.
pub fn get_first_word_owned(string_input: &str) -> String {
    string_input
        .split_whitespace()
        .next()
        .unwrap_or("")
        .to_string()
}

/// Returns the first word as a borrowed string slice.
pub fn get_first_word_slice(string_input: &str) -> &str {
    string_input.split_whitespace().next().unwrap_or("")
}
