use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::path::PathBuf;

pub fn read_file(mut str: &str) -> std::io::Result<()> {
    if str.is_empty() {
        str = ".";
    }

    let path = PathBuf::from(str);
    let absolute_path = fs::canonicalize(&path)?;

    println!("Absolute path: {:?}", absolute_path);

    let file = fs::File::open(&absolute_path);
    let reader = BufReader::new(file?);

    for line in reader.lines() {
        let line = line?;
        println!("{}", line);
    }
    Ok(())
}
