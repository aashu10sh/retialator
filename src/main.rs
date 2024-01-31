use std::env::args;
use std::process;

use retialator::{change_extension, create_backup};
fn main() {
    let arguments: Vec<String> = args().collect();
    if arguments.len() != 2 {
        eprintln!("Usage : ./{} directory_name", arguments[0]);
        process::exit(1);
    }
    let directory_name = arguments[1].as_str();

    let files = retialator::get_files_in_directory(directory_name).unwrap_or_else(|error| {
        eprintln!("{}", error);
        process::exit(1);
    });
    create_backup(&files);
    change_extension(&files, &directory_name);
    for file in files {
        println!("Ducked {}", file.original);
    }

    // println!("Hello, world!");
}
