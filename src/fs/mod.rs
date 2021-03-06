use std::os::unix::fs::MetadataExt;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

pub mod dir;
pub mod file;

pub fn info(path: &str) -> String {
    let file = Path::new(path);
    if let Err(err) = file.metadata() {
        return err.to_string();
    }
    let meta = file.metadata().unwrap();
    let perms = meta.permissions();
    let size_kb = meta.size() as f64 / 1024f64;
    format!(
        "Size: {}
Symlink: {}
Directory: {}
Permissions: {:?}
Read-only: {}
Time since modification: {:?}s
Time since accessed: {:?}s",
        if size_kb as u64 > 1024 {
            format!("{:.2}mb", size_kb / 1024f64)
        } else {
            format!("{:.2}kb", size_kb)
        },
        meta.file_type().is_symlink(),
        file.is_dir(),
        meta.permissions().mode(),
        perms.readonly(),
        meta.modified().unwrap().elapsed().unwrap().as_secs() as u32,
        meta.accessed().unwrap().elapsed().unwrap().as_secs() as u32,
    )
}

pub fn rename(path: &str, new_name: &str) -> Result<(), std::io::Error> {
    if let Err(err) = std::fs::rename(&path, new_name) {
        return Err(err);
    }
    Ok(())
}

pub fn delete(path: &str) -> Result<(), std::io::Error> {
    match Path::new(&path).is_dir() {
        true => {
            if let Err(err) = std::fs::remove_dir_all(&path) {
                return Err(err);
            }
        }
        false => {
            if let Err(err) = std::fs::remove_file(&path) {
                return Err(err);
            }
        }
    }
    Ok(())
}
