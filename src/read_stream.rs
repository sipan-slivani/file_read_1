use std::io;

pub fn stream_to_int(str: &str) -> i32 {
    println!("Stream to integer:{}", str);

    let mut input = String::new();
    let mut kk = -1;
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            // Attempt to parse the input as an i32
            match input.trim().parse::<i32>() {
                Ok(number) => {
                    //kk=number;
                    // println!("You entered: {}", number);
                    kk = number;
                }
                Err(e) => {
                    eprintln!("Error parsing input as an integer: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Error reading input: {}", e);
        }
    }

    kk
}

pub fn stream_to_string(str: &str) -> String {
    println!("stream to String:{}:", str);

    let mut input = String::new();
    let mut kk = "".to_string();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            // Attempt to parse the input as an i32
            match input.trim().parse::<String>() {
                Ok(number) => {
                    //kk=number;
                    // println!("You entered: {}", number);
                    kk = number;
                }
                Err(e) => {
                    eprintln!("Error parsing input as an integer: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Error reading input: {}", e);
        }
    }

    kk
}
