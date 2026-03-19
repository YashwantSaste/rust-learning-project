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