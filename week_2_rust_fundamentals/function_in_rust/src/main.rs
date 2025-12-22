use std::io;

fn process_numbers(numbers: &[i32]) {
    // Initialize the sum to zero
    let mut sum = 0;

    // Iterate over the numbers, adding each one to the sum
    for number in numbers {
        sum += number;
    }

    // Print the sum
    println!("The sum of the numbers is: {}", sum);

    // If the sum is even, print a message
    if sum % 2 == 0 {
        println!("The sum is even");
    } else {
        println!("The sum is odd");
    }
}


fn split_string(s: String, delimiter: char, field: usize) -> String {
    let parts: Vec<&str> = s.split(delimiter).collect();
    let result = parts.get(field);

    // This will not compile!
    result.expect("oops something went wrong").to_string()
}

fn sum(numbers: &[i32]) -> i32 {
    let mut total = 0;
    for n in numbers {
        total += n;
    }
    total
}

fn read_i32(prompt: &str) -> i32 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        match input.trim().parse::<i32>() {
            Ok(value) => return value,
            Err(_) => println!("Please enter a valid integer."),
        }
    }
}

fn read_usize(prompt: &str) -> usize {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        match input.trim().parse::<usize>() {
            Ok(value) => return value,
            Err(_) => println!  ("Please enter a valid positive whole number."),
        }
    }
}

fn main() {
    process_numbers(&[1, 2, 3]);
    let chunk = split_string("hello,world".to_string(), ',', 1);
    println!("Split): {}", chunk);
    let count = read_usize("How many numbers will you enter?");

    let mut numbers: Vec<i32> = Vec::new();
    for i in 0..count {
        let value = read_i32(&format!("Enter number {}:", i + 1));
        numbers.push(value);
    }

    let total = sum(&numbers);
    println!("The sum is: {}", total);
}


