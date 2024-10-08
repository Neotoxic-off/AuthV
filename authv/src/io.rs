use std::path::Path;
use std::fs::{self, File};
use std::io::{self, Read};
use sha2::{Sha256, Digest};
use std::collections::HashMap;

pub fn hash(content: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(content);
    hasher.finalize().into()
}

pub fn is_directory(path: &String) -> bool {
    return Path::new(path).exists() && Path::new(path).is_dir();
}

pub fn get_directory(directory: &String) -> HashMap<String, String> {
    let mut content: HashMap<String, String> = HashMap::new();
    let entries = fs::read_dir(directory).unwrap();

    for entry in entries {
        let path: std::path::PathBuf = entry.unwrap().path();
        let path_str = path.to_string_lossy().to_string();
        let file_name = path.file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();

        if path.is_dir() {
            content.extend(get_directory(&path_str)); 
        } else {
            content.insert(file_name, path_str);
        }
    }

    return content;
}

pub fn open_file(path: &str) -> io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut contents = Vec::new();

    file.read_to_end(&mut contents)?;

    return Ok(contents);
}
