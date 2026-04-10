//! Core data types used across the tutorial project.
//!
//! This module holds the structs and enums used by multiple examples, along
//! with the small demonstrations for methods, enums, and pattern matching.

use std::fmt;

#[derive(Debug)]
pub struct User {
    pub name: String,
    pub age: u32,
    pub email: String,
    pub active: bool,
}

impl User {
    pub fn new(name: impl Into<String>, age: u32, email: impl Into<String>, active: bool) -> Self {
        Self {
            name: name.into(),
            age,
            email: email.into(),
            active,
        }
    }
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "User {{ name: {}, age: {}, email: {}, active: {} }}",
            self.name, self.age, self.email, self.active
        )
    }
}

pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    pub fn perimeter(&self) -> u32 {
        (self.width * 2) + (self.height * 2)
    }
}

#[derive(Copy, Clone)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

pub enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Square(f64),
}

/// Runs the examples for structs, methods, and enums.
pub fn run() -> User {
    println!("== Structs and Enums ==");

    let user = User::new("Alice Borderland", 30, "alice@borderland.in", true);
    println!(
        "User details: Name: {}, Age: {}, Email: {}, Active: {}",
        user.name, user.age, user.email, user.active
    );

    let rectangle = Rectangle {
        width: 10,
        height: 5,
    };
    println!("Area of the rectangle is: {}", rectangle.area());
    println!("Perimeter of the rectangle is: {}", rectangle.perimeter());

    for direction in [
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ] {
        change_direction(direction);
    }

    let circle = Shape::Circle(5.0);
    let rectangle_shape = Shape::Rectangle(10.0, 5.0);
    let square = Shape::Square(4.0);
    println!("Area of the circle is: {}", calculate_area(circle));
    println!(
        "Area of the rectangle is: {}",
        calculate_area(rectangle_shape)
    );
    println!("Area of the square is: {}", calculate_area(square));

    user
}

fn change_direction(direction: Direction) {
    match direction {
        Direction::North => println!("Going North!"),
        Direction::South => println!("Going South!"),
        Direction::East => println!("Going East!"),
        Direction::West => println!("Going West!"),
    }
}

fn calculate_area(shape: Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
        Shape::Rectangle(width, height) => width * height,
        Shape::Square(side) => side * side,
    }
}
