// use std::env::join_paths;
use std::fs::File as SystemFile;
use std::io::{BufWriter, Write};
use std::{fs, path::Path};

pub struct File<'a> {
    pub original: String,
    pub new: Option<&'a str>,
}

impl<'a> File<'a> {
    pub fn new(file_name: String) -> Self {
        Self {
            original: file_name,
            new: None,
        }
    }
    pub fn set_name(&'a mut self, new_file_name: &'a str) {
        self.new = Some(new_file_name);
    }
}

pub fn get_files_in_directory(folder_name: &str) -> Result<Vec<File>, &str> {
    let folders_or_error = fs::read_dir(folder_name);
    let folders = match folders_or_error {
        Ok(entries) => entries,
        Err(_) => return Err("Could not iterate directories."),
    };
    let mut files: Vec<File> = Vec::new();
    for folder in folders {
        let file = folder.expect("Cant Access this Folder");
        let file_name = file
            .file_name()
            .into_string()
            .expect("Cannot get the File Name to String.");
        let file = File::new(file_name.to_owned());
        files.push(file);
    }
    Ok(files)
}

pub fn create_backup(files: &Vec<File>, directory: &str) {
    let backup_path = Path::new(directory).join("backup.text");
    let backup = SystemFile::create(backup_path).expect("Could not create File!");
    let mut backup_writer = BufWriter::new(backup);

    for file in files {
        backup_writer
            .write_fmt(format_args!("{}\n", file.original))
            .expect("Could not Write Original FIle Names");
    }
    backup_writer.flush().expect("Could not close File");
}

pub fn change_extension(files: &Vec<File>, directory: &str) -> bool {
    for file in files {
        let path = Path::new(directory).join(file.original.as_str());
        let new_path: Vec<&str> = path
            .to_str()
            .expect("Cannot Split the Path")
            .split_terminator(".")
            .collect();
        let to_rename = new_path[0].to_owned() + ".cageahahahaha???";

        let result = fs::rename(path, to_rename);
        match result {
            Ok(_) => {
                println!("Destroyed {:?}", file.original);
                // return true;
            }
            Err(error) => {
                println!("Error is :{}", error);
                return false;
            }
        }
    }
    true
}
