mod csv_read;
mod folder_contain;
mod read_file;
mod read_stream;

use crate::csv_read::csv_r;
use crate::folder_contain::folder_con;
use crate::read_stream::{stream_to_int, stream_to_string};

use crate::read_file::read_file;

fn main() {
    println!();
    let ext = stream_to_string("extension:");
    println!();
    let path = stream_to_string("path:");

    let v1 = folder_con(path, ext.as_str());
    for (i, j) in v1.iter().enumerate() {
        println!("{}  -  {}", i, j);
    }

    let num = stream_to_int("file index:").unwrap();

    let v1 = csv_r(v1[num as usize].clone()).expect("csv reading fail");

    for i in v1 {
        print!("  {i}  ");
    }
}
