//! Ownership and move semantics examples.
//!
//! The examples here show how ownership is moved, cloned, borrowed, and
//! returned back to the caller.

/// Runs the ownership-related examples.
pub fn run() {
    println!("== Ownership ==");

    let moved_string = String::from("hello");
    let new_owner = moved_string;
    println!("Moved string now owned by: {}", new_owner);

    let original = String::from("hello");
    let cloned = original.clone();
    println!("Cloned values: {} and {}", original, cloned);

    let returned_string = takes_ownership_returns_string(String::from("hello"));
    println!("Returned ownership: {}", returned_string);

    takes_ownership(String::from("temporary ownership"));
}

fn takes_ownership_returns_string(some_string: String) -> String {
    println!("{}", some_string);
    some_string
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}
