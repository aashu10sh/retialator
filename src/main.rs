use std::env::args;
use std::process;

use retialator::{change_extension, create_backup, get_files_in_directory};
fn main() {
    let arguments: Vec<String> = args().collect();
    if arguments.len() != 2 {
        eprintln!("Usage : ./{} directory_name", arguments[0]);
        process::exit(1);
    }
    let directory_name = arguments[1].as_str();

    let files = get_files_in_directory(directory_name).unwrap_or_else(|error| {
        eprintln!("{}", error);
        process::exit(1);
    });
    create_backup(&files, directory_name);
    match change_extension(&files, &directory_name) {
        true => {
            println!("YOU DO NOT MESS WITH THE RETIALATOR")
        }
        false => {
            println!("Sadge :(")
        }
    }
    // println!("Hello, world!");
}
