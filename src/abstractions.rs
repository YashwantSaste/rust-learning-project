//! Generics, traits, and lifetime examples.
//!
//! This module keeps the more abstract language features together so the rest
//! of the project can reuse the shared `User` type without duplicating logic.

use crate::models::User;

pub trait Summary {
    fn summarize(&self) -> String;
}

impl Summary for User {
    fn summarize(&self) -> String {
        format!(
            "{} is {} years old and can be contacted at {}",
            self.name, self.age, self.email
        )
    }
}

struct Fix;

impl Summary for Fix {
    fn summarize(&self) -> String {
        String::from("This is a fixed summary.")
    }
}

pub struct UserWithLifetime<'a> {
    pub name: &'a str,
}

/// Runs the generics, traits, and lifetime examples.
pub fn run(user: &User) {
    println!("== Generics, Traits, and Lifetimes ==");

    let first_generic = generic_function(5);
    let second_generic = generic_function("Hello");
    println!("First generic: {}", first_generic);
    println!("Second generic: {}", second_generic);

    println!("User summary: {}", user.summarize());

    let fix = Fix;
    notify(&fix);
    notify_as_detailed_syntax(user);
    notify_with_multiple_traits(user);

    let owned_longest = longest_string_owned(String::from("Hello"), String::from("Hello Rust"));
    println!("Longest owned string: {}", owned_longest);

    let str1 = String::from("Hello");
    let str2 = String::from("Hello Rust");
    let longest = longest_string_slice(&str1, &str2);
    println!("Longest string slice: {}", longest);

    let user_with_lifetime = UserWithLifetime {
        name: "Alice Borderland",
    };
    println!("User with lifetime: {}", user_with_lifetime.name);
}

fn longest_string_owned(str1: String, str2: String) -> String {
    if str1.len() > str2.len() { str1 } else { str2 }
}

fn longest_string_slice<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() { str1 } else { str2 }
}

fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn notify_as_detailed_syntax<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

fn notify_with_multiple_traits<T: Summary + std::fmt::Display>(item: &T) {
    println!("Breaking news! {}", item.summarize());
    println!("Display output: {}", item);
}

fn generic_function<T>(input: T) -> T {
    input
}
