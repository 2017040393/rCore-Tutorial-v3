use std::{env, fs};

fn main() {
    let cur_dir = env::current_dir().unwrap();

    println!("Current directory: {}", cur_dir.display());

    for entry in fs::read_dir(cur_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            println!("Directory: {}", path.display());
        } else {
            println!("File: {}", path.display());
        }
    }
}
