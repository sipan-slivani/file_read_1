use std::io;
use std::num::ParseIntError;

pub fn stream_to_int(str: &str) -> Result<i32, ParseIntError> {
    println!("Stream to integer:{}", str);

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("not reading stream integer");
    let kk = input.trim().parse::<i32>()?;

    Ok(kk)
}

pub fn stream_to_string(str: &str) -> String {
    println!("stream to String:{}:", str);

    let mut input = String::new();
    let mut kk = "".to_string();
    let ss = io::stdin()
        .read_line(&mut input)
        .expect("not reading String Stream");
    kk = input.trim().parse::<String>().unwrap();
    println!("{ss}");

    kk
}
