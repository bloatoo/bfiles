use std::fs;
use std::io;

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
pub fn create(name: &str) -> Result<(), io::Error> {
    let file = std::fs::File::create(&name);
    if let Err(err) = file {
        return Err(err);
    }
    Ok(())
}
