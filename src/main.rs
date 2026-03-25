use core::hash;
use std::fs;

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

fn get_first_word(sentence: String)->String {
    let mut answer = String::from("");
    for char in sentence.chars() {
        answer.push_str(char.to_string().as_str());
        if char == ' ' {
            break;
        }
    }
    return answer;
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