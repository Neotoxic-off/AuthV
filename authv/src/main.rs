use env_logger;
use hex;

use std::env;
use log::{info, error, warn};
use std::collections::HashMap;

pub mod io;

fn valid_arguments(arguments: &Vec<String>) -> bool {
    if arguments.len() != 2 {
        error!("Not the good number of arguments: {} <directory>", arguments[0]);
        return false;
    }

    return true;
}

fn get_directory(arguments: &Vec<String>) -> String {
    return arguments[1].clone();
}

fn setup() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("trace")).init();
}

fn information(content: HashMap<String, String>) {
    info!("Element in directory: {}", content.len());
}

fn hash_files(content: HashMap<String, String>) -> HashMap<String, String> {
    let mut hash_table: HashMap<String, String> = HashMap::new();

    warn!("Hashing the files");
    for (name, path) in content.iter() {
        match io::open_file(path) {
            Ok(file_content) => {
                let element_hash = io::hash(&file_content);
                let element_hash_hex = hex::encode(element_hash);
                hash_table.insert(path.clone(), element_hash_hex);
            }
            Err(e) => {
                error!("Error reading file '{}': {}", path, e);
            }
        }
    }

    return hash_table;
}

fn has_hash_table(path: String) -> bool {
    return io::is_file(&path);
}

fn compare_tables(stored: HashMap<String, String>, current: HashMap<String, String>) -> Result<(), Box<dyn std::error::Error>> {
    for (path, stored_hash) in &stored {
        match current.get(path) {
            Some(current_hash) => {
                if current_hash != stored_hash {
                    error!("{} has been edited (Stored: {}, Current: {})", path, stored_hash, current_hash);
                } else {
                    info!("{} is intact", path);
                }
            }
            None => {
                error!("{} has been removed from current", path);
            }
        }
    }

    for (path, current_hash) in &current {
        if !stored.contains_key(path) {
            error!("{} has been added", path);
        }
    }

    Ok(())
}

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let hash_table_path: String = String::from("table.avht");
    let mut directory: String = String::new();
    let mut content: HashMap<String, String> = HashMap::new();
    let mut hash_table: HashMap<String, String> = HashMap::new();

    setup();

    if valid_arguments(&arguments) {
        directory = get_directory(&arguments);
        if io::is_directory(&directory) == true {
            info!("Directory set to: {}", directory);
            content = io::get_directory(&directory);
            information(content.clone());
            hash_table = hash_files(content.clone());
            if has_hash_table(hash_table_path.clone()) == true {
                compare_tables(io::load_json_file(hash_table_path.clone()).unwrap(), hash_table.clone());
            } else {
                io::save_json_file(hash_table_path.clone(), hash_table);
                info!("Saved hash table at: {}", hash_table_path);
            }
        } else {
            error!("Directory '{}' not found", directory);
        }
    }
}
