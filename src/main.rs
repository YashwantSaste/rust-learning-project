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

    stack_fn();   // Call the function that uses stack memory
    heap_fn();    // Call the function that uses heap memory
    update_string();  // Call the function that changes size of variable at runtime
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
