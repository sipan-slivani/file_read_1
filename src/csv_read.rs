use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn csv_r(path: String) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut v1: Vec<String> = vec![];
    for line in reader.lines() {
        let line = line?;
        let cells: Vec<&str> = line.split(',').collect();

        // Now you can access the cells in the row
        for cell in cells {
            v1.push(cell.to_string());
            print!("   {}   ", cell);
        }
        v1.push("\n".to_string());
        println!();
    }
    Ok(v1)
}
