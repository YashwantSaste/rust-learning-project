//! Basic Rust syntax examples.
//!
//! This module groups together introductory examples for variables, control
//! flow, loops, stack and heap allocation, and simple helper functions.

use crate::text;

/// Runs the examples covered in the basics section.
pub fn run() {
    println!("== Basics ==");

    let x: i32 = 5;
    let num = 32;
    println!("{}", num);
    println!("{}", x);

    let is_odd = true;
    if is_odd {
        println!("The given number is odd");
    }

    let greeting = String::from("Hello Rust!");
    println!("{}", greeting);

    match greeting.chars().nth(2) {
        Some(character) => println!("The char at the index is: {}", character),
        None => println!("Out of bound!"),
    }

    for number in 0..10 {
        println!("{}", number);
    }

    let sentence = String::from("This is the initializing statement");
    let first_word = text::get_first_word_owned(&sentence);
    println!("First word is: {}", first_word);

    let first = 1;
    let second = 3;
    println!("{}", sum(first, second));
    println!("Hello, world!");

    stack_fn();
    heap_fn();
    inspect_string_growth(3);
}

fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn stack_fn() {
    let a = 10;
    let b = 20;
    let c = a + b;
    println!("Stack function: The sum of {} and {} is {}", a, b, c);
}

fn heap_fn() {
    let s1 = String::from("Hello");
    let s2 = String::from("World");
    let combined = format!("{} {}", s1, s2);
    println!("Heap function: Combined string is '{}'", combined);
}

fn inspect_string_growth(iterations: usize) {
    let mut text = String::from("Initial string");
    println!("Before update: {}", text);
    println!(
        "Capacity: {}, Length: {}, Pointer: {}",
        text.capacity(),
        text.len(),
        text.as_ptr() as usize
    );

    for _ in 0..iterations {
        text.push_str(" and some additional text");
        println!(
            "Capacity: {}, Length: {}, Pointer: {}",
            text.capacity(),
            text.len(),
            text.as_ptr() as usize
        );
    }
}
