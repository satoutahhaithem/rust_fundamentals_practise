use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();

    // Require a file path argument
    if args.len() < 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        return;
    }

    let file_path = &args[1];

    // Open the file with match-based error handling
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => {
                eprintln!("Error: File not found -> {}", file_path);
                return;
            }
            io::ErrorKind::PermissionDenied => {
                eprintln!("Error: Permission denied -> {}", file_path);
                return;
            }
            _ => {
                eprintln!("Error: Could not open file {} -> {}", file_path, error);
                return;
            }
        },
    };

    // Read and print lines
    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(text) => println!("{}", text),
            Err(error) => {
                eprintln!("Error reading a line: {}", error);
                return;
            }
        }
    }
}
