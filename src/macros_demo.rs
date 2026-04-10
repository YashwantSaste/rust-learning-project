//! Formatting and macro examples.
//!
//! The `println!` examples in this module focus on how formatting traits such
//! as `Display` and `Debug` affect the output of user-defined types.

use crate::models::User;

/// Runs the formatting examples for custom types.
pub fn run(user: &User) {
    println!("== Formatting and Macros ==");
    println!("Printing struct with Display trait: {}", user);
    println!("Printing struct with Debug trait: {:?}", user);
}
