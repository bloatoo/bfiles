use std::fs;
pub fn read(path: &str) -> Vec<String> {
    let dir_contents = fs::read_dir(path);

    if let Err(err) = dir_contents {
        return vec![String::from(err.to_string())];
    }

    let mut result: Vec<String> = dir_contents
        .unwrap()
        .map(|key| key.unwrap().file_name().to_str().unwrap_or("Filename not valid unicode").to_string())
        .collect();

    result.sort();

    if result.is_empty() {
        return vec![String::from("No files in directory")];
    }
    result
}
