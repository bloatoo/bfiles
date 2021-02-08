use std::fs;
pub fn read(path: &str) -> String {
    let read_file = fs::read_to_string(path);
    let contents;
    if let Err(err) = read_file {
        contents = err.to_string();
    } else {
        contents = read_file.unwrap();
    }
    contents
}
