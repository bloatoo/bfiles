use std::fs;
pub fn read(path: &str) -> Vec<String> {
    let dir_contents = fs::read_dir(path);

    if let Err(err) = dir_contents {
        return vec![err.to_string()];
    }

    dir_contents.unwrap().map(|key| {
        format!("{}", key.unwrap()
                .path()
                .to_str()
                .unwrap()
        )
    }).collect()
}
