use std::fs;
use std::path::PathBuf;

pub fn folder_con(folder_path: String, filter: &str) -> Vec<String> {
    let mut v1: Vec<String> = vec![];
    // Read the contents of the folder

    let entries = fs::read_dir(&folder_path).unwrap();

    for entry in entries.flatten() {
        // let ft = entry;
        //let name = entry.file_name().to_string_lossy().to_string();
        if entry.path().is_file() {
            if let Some(fx) = entry.path().extension() {
                if filter.is_empty() || fx.to_string_lossy() == filter {
                    let p = entry.path().to_string_lossy().to_string();

                    //let path = PathBuf::from(p.clone());
                    let absolute_path = fs::canonicalize(p.to_string())
                        .unwrap()
                        .to_string_lossy()
                        .to_string();
                    v1.push(absolute_path.clone());
                    println!("aaaabbbbolut   ->   {absolute_path}");
                }
            }
        }
    }
    v1
}
