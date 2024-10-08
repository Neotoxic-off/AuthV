use std::env;
use log::{info, error};
use env_logger;

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

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let directory: String;

    setup();

    if valid_arguments(&arguments) {
        directory = get_directory(&arguments);
        info!("Directory set to: {}", directory);
    } else {
        error!("Invalid arguments.");
    }
}
