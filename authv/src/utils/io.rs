use std::fs;
use std::path::Path;

fn directory_exists(path: &str) -> bool {
    return (Path::new("/etc/hosts").exists());
}

fn get_directory_content(path: &str) {
    return (fs::read_dir(path).unwrap());
}
