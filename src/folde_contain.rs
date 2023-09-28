use std::fs;

pub fn folder_con(mut folder_path: String, filter: &str) -> Vec<String> {
    // Specify the path to the folder you want to list files in
    //folder_path = ".".to_string(); // Change this to the path of your folder

    let mut v1: Vec<String> = vec![];

    // Read the contents of the folder

    let entries = fs::read_dir(folder_path).unwrap();

    for entry in entries {
        if let Ok(ft) = entry {
            let name = ft.file_name().to_string_lossy().to_string();
            if ft.path().is_file() {
                if let Some(fx) = ft.path().extension() {
                    if filter == "" {
                        v1.push(name)
                    } else if fx.to_string_lossy().to_string() == filter {
                        v1.push(name)
                    }
                }
            }
        }
        // if true
        // //entry.unwrap().path().clone().is_file()&& entry.unwrap().path().extension().unwrap().clone() == filter
        // {
        //
        //     let ex= entry.unwrap().path().file_name().unwrap().to_string_lossy().to_string().clone();
        //    // println!("{}",ex.clone());
        //     v1.push(entry.unwrap().file_name().to_string_lossy().to_string());
        // }
    }
    v1
}
