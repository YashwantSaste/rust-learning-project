//! Iterator and adapter examples.
//!
//! This module shows immutable iteration, mutable iteration, consuming
//! iterators, and common adapters such as `map` and `filter`.

/// Runs the iterator demonstrations.
pub fn run() {
    println!("== Iterators ==");

    let iterator_vector = vec![1, 2, 3, 4, 5];
    for number in iterator_vector.iter() {
        println!("Iterating over vector: {}", number);
    }
    println!("Original vector after iteration: {:?}", iterator_vector);

    let mut mutable_vector = vec![1, 2, 3, 4, 5];
    for number in mutable_vector.iter_mut() {
        *number += 10;
    }
    println!("Mutable vector after mutation: {:?}", mutable_vector);

    let mut iterator_vector2 = vec![1, 2, 3, 4, 5];
    let iterator2 = iterator_vector2.iter_mut();
    for value in iterator2 {
        println!("Iterating with mutable references: {}", value);
    }

    let into_iterator_vector = vec![1, 2, 3, 4, 5];
    for number in into_iterator_vector.into_iter() {
        println!("Iterating with into_iter: {}", number);
    }

    let consumer_vector = vec![1, 2, 3, 4, 5];
    let sum: i32 = consumer_vector.into_iter().sum();
    println!("Sum of the consumer vector: {}", sum);

    let adapter_vector = vec![1, 2, 3, 4, 5];
    let adapter_iterator = adapter_vector.iter().map(|x| x * 2).filter(|x| *x > 5);
    for number in adapter_iterator {
        println!("Iterating with adapter: {}", number);
    }
    println!("Original vector after adapter: {:?}", adapter_vector);

    let filter_vector = vec![1, 2, 3, 4, 5];
    let filter_iterator = filter_vector.iter().filter(|x| **x % 2 == 0);
    for number in filter_iterator {
        println!("Iterating with filter: {}", number);
    }

    let map_vector = vec![1, 2, 3, 4, 5];
    let filtered_and_doubled = filter_odd_and_double(map_vector);
    for number in filtered_and_doubled {
        println!("Iterating with filter and map: {}", number);
    }
}

fn filter_odd_and_double(number_vector: Vec<i32>) -> Vec<i32> {
    number_vector
        .into_iter()
        .filter(|number| number % 2 != 0)
        .map(|number| number * 2)
        .collect()
}
