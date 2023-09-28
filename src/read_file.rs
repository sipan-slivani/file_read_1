use std::fs::File;
use std::io::Read;

pub fn read_file(str: &str) {
    // Open the file for reading
    let mut file = match File::open(str) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            return;
        }
    };

    // Create a buffer to store the file contents
    let mut contents = String::new();

    // Read the file contents into the buffer
    match file.read_to_string(&mut contents) {
        Ok(_) => {
            // Successfully read the file
            println!("File contents:\n{}", contents);
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
        }
    }
}
