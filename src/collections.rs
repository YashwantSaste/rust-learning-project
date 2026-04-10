//! Collections, options, and basic error handling.
//!
//! These examples cover vectors, hash maps, `Option`, and matching on results
//! returned by file system APIs.

use std::{collections::HashMap, fs};

/// Runs the collection and error-handling examples.
pub fn run() {
    println!("== Collections and Error Handling ==");

    let result = fs::read_to_string("non-existent-path");
    match result {
        Ok(content) => println!("File content: {}", content),
        Err(error) => println!("Error reading file: {}", error),
    }

    let index_of_a = find_first_a("This is a sample string");
    match index_of_a {
        Some(index) => println!("The index of first 'a' is: {}", index),
        None => println!("Character 'a' not found in the string."),
    }

    let mut vect = Vec::new();
    vect.push(1);
    vect.push(2);
    println!("Vector: {:?}", vect);

    let even_numbers = filter_even_numbers(vect);
    println!("Even numbers: {:?}", even_numbers);

    let vect2 = vec![1, 2, 3, 4, 5, 6];
    let even_numbers_ref = filter_even_numbers_by_reference(&vect2);
    println!("Even numbers with reference: {:?}", even_numbers_ref);
    println!("Original vector after reference function: {:?}", vect2);

    let vect3 = vec![10, 15, 20, 25, 30];
    println!("Original vector: {:?}", vect3);

    let mut hashmap = HashMap::new();
    hashmap.insert("John Doe", 33);
    hashmap.insert("Alice Borderland", 30);
    println!("HashMap: {:?}", hashmap);
    match hashmap.get("John Doe") {
        Some(age) => println!("The age of John Doe is: {}", age),
        None => println!("John Doe not found in the HashMap"),
    }

    let input_vector = vec![
        (String::from("John Doe"), 33),
        (String::from("Alice Borderland"), 30),
    ];
    let grouped_hashmap = group_values_by_key(input_vector);
    println!("Grouped HashMap: {:?}", grouped_hashmap);
}

fn group_values_by_key(pairs: Vec<(String, i32)>) -> HashMap<String, Vec<i32>> {
    let mut hashmap: HashMap<String, Vec<i32>> = HashMap::new();
    for (key, value) in pairs {
        hashmap.entry(key).or_default().push(value);
    }
    hashmap
}

fn filter_even_numbers(numbers: Vec<i32>) -> Vec<i32> {
    numbers
        .into_iter()
        .filter(|number| number % 2 == 0)
        .collect()
}

fn filter_even_numbers_by_reference(numbers: &[i32]) -> Vec<i32> {
    numbers
        .iter()
        .copied()
        .filter(|number| number % 2 == 0)
        .collect()
}

fn find_first_a(value: &str) -> Option<usize> {
    value.chars().position(|character| character == 'a')
}
