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
