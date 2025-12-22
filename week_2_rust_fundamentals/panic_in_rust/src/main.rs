use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() {
    let file = File::open("non_existent_file.txt");

    let file = match file {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                io::ErrorKind::NotFound => {
                    panic!("File not found: {}", error);
                }
                _ => {
                    panic!("Error opening file: {}", error);
                }
            }
        }
    };

    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line.unwrap());
    }
}
