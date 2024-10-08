use env_logger;
use hex;

use std::env;
use log::{info, error};
use std::collections::HashMap;

pub mod io;

fn valid_arguments(arguments: &Vec<String>) -> bool {
    if arguments.len() != 2 {
        error!("Not enough arguments: {} <directory>", arguments[0]);
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

    for (name, path) in content.iter() {
        match io::open_file(path) {
            Ok(file_content) => {
                let element_hash = io::hash(&file_content);
                let element_hash_hex = hex::encode(element_hash);
                info!("{}", element_hash_hex);
                hash_table.insert(path.clone(), element_hash_hex);
            }
            Err(e) => {
                error!("Error reading file '{}': {}", path, e);
            }
        }
    }

    return hash_table;
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
            io::save_file(hash_table_path.clone(), hash_table);
            info!("Saved hash table at: {}", hash_table_path);
        } else {
            error!("Directory '{}' not found", directory);
        }
    }
}
