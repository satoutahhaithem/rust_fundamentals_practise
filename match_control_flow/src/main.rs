use std::io;

fn main() {
    println!("Please enter a greeting:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read input");

    let greeting = name.trim().to_lowercase();

    // use of match expression to pattern match against variable "name"
    match greeting.trim() {
        "good bye" => println!("Sorry to see you go."),
        "hello" => println!("Hi, nice to meet you!"),
        "good morning" => println!("Good morning"),
        "good evening" => println!("Good evening"),
        _ => println!("I can't find a greeting, good bye."),
    }
}

