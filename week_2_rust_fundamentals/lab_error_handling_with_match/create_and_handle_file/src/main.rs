use std::fs::File;
use std::io::Write;

fn write_to_file(filename: &str, content: &str) {
    let file = File::create(filename);

    let mut file = match file {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::PermissionDenied => {
                    panic!("Permission denied when creating file: {}", error)
                }
                _ => {
                    panic!("Error creating file: {}", error)
                }
            }
        }
    };

    let write_result = file.write_all(content.as_bytes());

    match write_result {
        Ok(_) => println!("Successfully wrote to the file."),
        Err(error) => {
            panic!("Error writing to file: {}", error)
        }
    }
}

fn main() {
    write_to_file("output.txt", "Hello, Rust error handling!");
}
