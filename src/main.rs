use core::hash;
use std::{char, fs, iter::Sum, string};

fn main() {

    //initializing an integer type of variable in the Rust..
    let x :i32= 5;
    let num =32;
    println!("{}",num);
    println!("{}", x);

    //initializing a boolean type of variable in the Rust..
    let is_odd = true;
    if is_odd {
        println!("The given number is odd");
    }

    //initializing a boolean type of variable in the Rust..
    let str = String::from("Hello Rust!");
    println!("{}",str);

    // indexing a string 

    let char1 = str.chars().nth(2);
    match char1 {
        Some(char1)=>println!("The char at the index is: {}",char1),
        None=>println!("Out of bound!"),
    }

    //For loop
    for num in 0..10 {
        println!("{}", num);
    }

    let sentence:String = String::from("This is the initializing statement");
    let first_word: String = get_first_word(sentence);
    println!("First word is: {}", first_word);

   // stack_fn();   // Call the function that uses stack memory
    //heap_fn();    // Call the function that uses heap memory
    //update_string();  // Call the function that changes size of variable at runtime

    let x = 1; // crated on stack
	let y = 3; // created on stack
    println!("{}", sum(x, y));
    println!("Hello, world!");

    let s1 = String::from("hello");
    let s2 = s1;
    // This line would cause a compile error because ownership has been moved.
    // println!("{}", s1); 

    // This line would work because we have cloned the string, so both s1 and s2 own their own data.
    //let s2 = s1.clone();


    let my_string = String::from("hello");
    takes_ownership(my_string);
    // println!("{}", my_string); // This line would cause a compile error because ownership has been moved.

    // To return ownership back to the main function, we can modify the function to return the string.
    // let s2 = takes_ownership_returns_string(s1);

    // Structs in Rust
    let user = User {
        name : String::from("Alice Borderland"),
        age : 30,
        email : String::from("alice@borderland.in:"),
        active : true,
    };

    println!("User details: Name: {}, Age: {}, Email: {}, Active: {}", user.name, user.age, user.email, user.active);

    let rectangle = Rectangle {
        width: 10,
        height: 5,
    };

    println!("Area of the rectangle is: {}", rectangle.area());
    println!("Perimeter of the rectangle is: {}", rectangle.perimeter());

    // Enums in Rust
    let direction = Direction::North;
    change_direction(direction);

    let circle = Shapes::Circle(5.0);
    let rectangle = Shapes::Rectangle(10.0, 5.0);
    let square = Shapes::Square(4.0);
    println!("Area of the circle is: {}", calculate_area(circle));
    println!("Area of the rectangle is: {}", calculate_area(rectangle));
    println!("Area of the square is: {}", calculate_area(square));

    //Error handling in Rust
    let result = fs::read_to_string("non-existent-path");
    //println!("Result of reading file: {:?}", result);
    match result {
        Ok(content) => println!("File content: {}", content),
        Err(e) => println!("Error reading file: {}", e),
    }

    //Option enum in Rust
    let index_of_a = find_first_a(String::from("This is a sample string"));
    match index_of_a {
        Some(index) => println!("The index of first 'a' is: {}", index),
        None => println!("Character 'a' not found in the string."), 
    }


    // Vectors in Rust
    let mut vect = Vec::new();
    vect.push(1);
    vect.push(2);
    println!("Vector: {:?}", vect);

    let even_numbers = filter_even_number(vect);
    println!("Even numbers: {:?}", even_numbers);

    let vect2 = vec![1, 2, 3, 4, 5, 6];
    let even_numbers_ref = filter_even_numbers_with_reference(&vect2);
    println!("Even numbers with reference: {:?}", even_numbers_ref);
    println!("Original vector after reference function: {:?}", vect2);

    //Initializing a vector with macros
    let mut vect3 = vec![10, 15, 20, 25, 30];
    println!("Original vector: {:?}", vect3);

    // Hashmaps in Rust
    use std::collections::HashMap;
    let mut hashmap = HashMap::new();
    hashmap.insert("John Doe", 33);
    hashmap.insert("Alice Borderland", 30);
    println!("HashMap: {:?}", hashmap);
    match hashmap.get("John Doe") {
        Some(age) => println!("The age of John Doe is: {}", age),
        None => println!("John Doe not found in the HashMap"),
    }

    let mut input_vector = vec![(String::from("John Doe"), 33), (String::from("Alice Borderland"), 30)];
    let grouped_hashmap = group_values_by_key(input_vector);
    println!("Grouped HashMap: {:?}", grouped_hashmap);


    let iterator_vector = vec![1, 2, 3, 4, 5];
    let iterator = iterator_vector.iter();
    for nums in iterator {
        println!("Iterating over vector: {}", nums);
    }


    // iterator does not take ownership of the vector, so we can still use it after the loop
    println!("Original vector after iteration: {:?}", iterator_vector);

    // cannot mutate the vector while iterating over it, as it would cause a compile error
    // as the iterator holds an immutable reference to the vector, so we cannot modify it while iterating.
    // for nums in iterator_vector.iter() {
    //     nums.push(10); // This line would cause a compile error
    // }

    // if we want to mutate the vector while iterating, we can use a mutable iterator
    let mut mutable_vector = vec![1, 2, 3, 4, 5];
    let mutable_iterator = mutable_vector.iter_mut();
    for nums in mutable_iterator {
        *nums += 10; // This line would work because we are using a mutable iterator
    }
    println!("Mutable vector after mutation: {:?}", mutable_vector);

    let mut iterator_vector2 = vec![1, 2, 3, 4, 5];
    let mut iterator2 = iterator_vector2.iter_mut();
    while let Some(val) = iterator2.next() {
        println!("Iterating with while let: {}", val);
    }

    // into_iter() takes ownership of the vector and returns an iterator that yields owned values, so we cannot use the original vector after calling into_iter()
    let into_iterator_vector = vec![1, 2, 3, 4, 5];
    let into_iterator = into_iterator_vector.into_iter();
    for nums in into_iterator {
        println!("Iterating with into_iter: {}", nums); 
    }

    // print!("Original vector after into_iter: {:?}", into_iterator_vector); // This line would cause a compile error because ownership has been moved.

    // Learning Consumer iterators in Rust
    // Consumer adapters are used when we want to consume the items of an iterator, meaning that we want to take ownership of the items and use them in some way.
    let consumer_vector = vec![1, 2, 3, 4, 5];
    let consumer_iterator = consumer_vector.into_iter();
    let sum: i32 = consumer_iterator.sum();
    println!("Sum of the consumer vector: {}", sum);
    // After consuming the vector with into_iter(), we cannot use the original vector anymore because ownership has been moved.
    // print!("Original vector after consuming: {:?}", consumer_vector); // This line would cause a compile error because ownership has been moved.

    // Learning Iterator adapters in Rust
    // Iterator adapters are used when we want to create a new iterator from an existing one, allowing us to perform transformations or filtering on the items without consuming them.
    let adapter_vector = vec![1, 2, 3, 4, 5];
    let adapter_iterator = adapter_vector.iter().map(|x| x * 2).filter(|x| *x > 5);
    for nums in adapter_iterator {
        println!("Iterating with adapter: {}", nums);   
    }

    // You can access the original vector after using iterator adapters because they do not consume the items, they just create a new iterator that yields transformed items.
    println!("Original vector after adapter: {:?}", adapter_vector);

    // Filter is an iterator adapter that allows us to filter items based on a predicate function. It takes a closure that returns a boolean value, and it yields only the items for which the closure returns true.
    let filter_vector = vec![1, 2, 3, 4, 5];
    let filter_iterator = filter_vector.iter().filter(|x| **x % 2 == 0);
    for nums in filter_iterator {
        println!("Iterating with filter: {}", nums);
    }

    // Map is an iterator adapter that allows us to transform items based on a mapping function. It takes a closure that returns a new value, and it yields the transformed items.
    let map_vector = vec![1, 2, 3, 4, 5];
    let filter_odd_and_double_iterator = filter_odd_and_double(map_vector);
    for nums in filter_odd_and_double_iterator {
        println!("Iterating with filter and map: {}", nums);
    }


    // Learning Strings and Slices in Rust
    let mut string1 = String::from("Hello");
    string1.push_str(" World");
    println!("String after push_str: {}", string1);
    // getting first word using method
    let first_word = get_first_word(string1.clone());
    println!("First word: {}", first_word);

    //using String slices to get the first word without taking ownership
    let string2 = String::from("Hello Rustaceans");
    let first_word_slice = get_first_word_using_slice(&string2);
    println!("First word using slice: {}", first_word_slice);


    // Three commonly used strings (there are acutally more)
    let first_type = String::from("Hello"); // This creates a String on the heap, which is growable and mutable.
    let second_type = "Hello"; // This creates a string slice (&str) which is a reference to a string literal, and it is immutable and has a fixed size.
    let third_type = &first_type; // This creates a string slice (&str) that references the String on the heap, allowing us to use it without taking ownership.

    // Slices can also be applied to arrays and vectors
    let array = [1, 2, 3, 4, 5];
    let slice = &array[1..4]; // This creates a slice that references the
    // elements from index 1 to 3 (inclusive) of the array, which are 2, 3, and 4.
    println!("Array slice: {:?}", slice);

    // Learing Generics in Rust
    // Generics allow us to write functions and data structures that can work with any type,
    // without having to specify the type explicitly. This is achieved using type parameters, which are placeholders for the actual types that will be used when the function or data structure is instantiated.

    let first_generic = generic_function(5); // This will work with an integer type
    let second_generic = generic_function("Hello"); // This will work with a string slice type
    println!("First generic: {}", first_generic);
    println!("Second generic: {}", second_generic);

    // Learning Traits in Rust
    // Traits are a way to define shared behavior in Rust. They allow us to specify a
    // set of methods that a type must implement in order to be considered as implementing that trait. This is similar to interfaces in other programming languages.

    println!("User summary: {}", user.summarize());

    let fix = Fix;
    notify(&fix);

    // Learning lifetimes in Rust
    let str1 = String::from("Hello");
    let str2 = String::from("Hello Rust");
    let longest = longest_string_slice(&str1, &str2);
    println!("Longest string slice: {}", longest);
}


// Write a function that takes two strings and returns longest of the two strings. 
// Approach 1: We can take ownership of both strings and return the longest one. 
// However, this approach is not efficient because we are taking ownership of both strings and returning a new string, which involves copying the data.
fn longest_string(str1: String, str2:String)->String{
    if str1.len() > str2.len() {
        return str1;
    }
    else{
        return str2;
    }
}

// Approach 2: We can instead return a string slice that references the original strings without taking ownership. 
// This way, we can avoid unnecessary copying and improve efficiency.
// Here 'a does not have to be the same as the lifetime of the input strings, but it must be at least as long as the lifetime of the 
// input strings, so that the returned string slice is valid for as long as the input strings are valid.
fn longest_string_slice<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() {
        return str1;
    } else {
        return str2;
    }
}

pub trait Summary {
    fn summarize(&self) -> String;
}

impl Summary for User {
    fn summarize(&self) -> String {
       return format!("{} is {} years old and can be contacted at {}", self.name, self.age, self.email);
    }
}

struct Fix;

impl Summary for Fix {
    fn summarize(&self) -> String {
        return String::from("This is a fixed summary.");
    }
}


// This is a sugar coat syntax for something different which is illustarted as method notify_as_detailed_syntax.
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Single trait bound
fn notify_as_detailed_syntx<T:Summary>(item: T){
    println!("Breaking news! {}", item.summarize());
}

// Multiple trait bounds
fn notify_with_multiple_traits<T:Summary + std::fmt::Display>(item: &T){
    println!("Breaking news! {}", item.summarize());
    println!("Display output: {}", item);
}

// Writing a function demonstarting the generics in Rust
fn generic_function<T>(input: T) -> T {
    return input;
}

fn get_first_word_using_slice(string_input:&String)->&str{
    let mut index =0;
    for (_,i) in string_input.chars().enumerate(){
        if i == ' ' {
            break;
        }
        index += 1;
    }
     return &string_input[0..index]
}
// Write a function that takes string as input and returns the first word of the string as output. If there is no space in the string, return the whole string as the first word.
// Problem in this approach is that we are taking ownership of the string input and returning a new string, which is not efficient. We can instead return a string slice that references the original string without taking ownership.
// We take up double the memory if the string1 gets cleared the answer string would still hold the value of the first word, which is not what we want. We can instead return a string slice that references the original string without taking ownership.
// What we want is a 'view' into the original string, and not to copy it

fn get_first_word(string_input:String)->String{
    let mut answer = String::from("");
    for char in string_input.chars() {
        if char == ' ' {
            break;
        }
        answer.push(char);
    }
    return answer;
}

// Writing a function that first filter all odd numbers and then double each value and create a new vector

fn filter_odd_and_double(number_vector:Vec<i32>)->Vec<i32>{
    return number_vector.iter().filter(|x| *x %2 !=0).map(|x| *x *2 ).collect();
}


// Wrtiting a function that takes a vector of tuples as an argument
// and returns a hashmap where the keys are the unique keys from the tuples and 
//the values are vectors of corresponding values from the tuples.

fn group_values_by_key(pairs: Vec<(String,i32)>) -> std::collections::HashMap<String, Vec<i32>>{
    let mut hashmap = std::collections::HashMap::new();
    for (key, value) in pairs {
        hashmap.entry(key).or_insert(Vec::new()).push(value);
    }
    return hashmap;
}

// Writing a function that takes a vector as an argument and returns a vector with even values
fn filter_even_number(numbers:Vec<i32>)->Vec<i32>{
    let mut answer_vector =Vec::new();
    for num in numbers {
        if num % 2 == 0 {
            answer_vector.push(num);
        }
    }
    return answer_vector;
}
// Writing a same function as above that takes a vector as an argument and returns
// a vector with even values but now passing the vector by reference instead of ownership transfer

fn filter_even_numbers_with_reference(numbers:&Vec<i32>)->Vec<i32>{
    let mut answer_vector =Vec::new();
    for num in numbers {
        if num % 2 == 0 {
            answer_vector.push(*num); // dereference the number to get the value
        }
    }
    return answer_vector;

}


fn find_first_a(str:String)->Option<i32>{
    for(index,char) in str.chars().enumerate(){
        if char == 'a' {
            return Some((index as i32));
        }
    }
    return None;
}
fn takes_ownership_returns_string(some_string: String) -> String {
    println!("{}", some_string); 
    return some_string; // return the string ownership back to the original main fn
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string); // `some_string` now owns the data.
}

fn sum(a: i32, b: i32) -> i32 {
    let c = a + b; 
    // a,b,c are created on stack and would be removed once the function call is over, as they are not needed anymore.
    return c;
}

fn stack_fn() {
    // Declare a few integers on the stack
    let a = 10;
    let b = 20;
    let c = a + b;
    println!("Stack function: The sum of {} and {} is {}", a, b, c);
}

fn heap_fn() {
    // Create a string, which is allocated on the heap
    let s1 = String::from("Hello");
    let s2 = String::from("World");
    let combined = format!("{} {}", s1, s2);
    println!("Heap function: Combined string is '{}'", combined);
}

fn update_string() {
    // Start with a base string on the heap
    let mut s = String::from("Initial string");
    println!("Before update: {}", s);
    println!("Capacity: {}, Length: {}, Pointer: {}", s.capacity(), s.len(), s.as_ptr() as usize);

    for _ in 0..100{
        // Append some text to the string
        s.push_str(" and some additional text");
        println!("Capacity: {}, Length: {}, Pointer: {}", s.capacity(), s.len(), s.as_ptr() as usize);
    };
}

struct User {
    name: String,
    age: u32,
    email: String,
    active: bool,
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn perimeter(&self)->u32{
        self.width * 2 + self.height * 2
    }
}

fn change_direction(dir:Direction){
    match dir{
        Direction::North => println!("Going North!"),
        Direction::South => println!("Going South!"),
        Direction::East => println!("Going East!"),
        Direction::West => println!("Going West!"),
    }
}
enum Direction {
    North,
    South,
    East,
    West,
}


// Enum with values
enum Shapes {
    Circle(f64), // radius
    Rectangle(f64, f64), // width, height
    Square(f64), // side length
}

fn calculate_area(shape:Shapes)->f64{
    match shape {
        Shapes::Circle(radius) => std::f64::consts::PI * radius * radius,
        Shapes::Rectangle(width, height) => width * height,
        Shapes::Square(side) => side * side,
    }
}