use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use crate::modules::file_system::{Utf8PathBuf, FileSystem, FileSystemError, FileSystemResult};

pub struct SystemFs;

impl SystemFs {
    fn new() -> FileSystemResult<SystemFs> {
        Ok(SystemFs)
    }
}

impl FileSystem for SystemFs {
    fn list_files_with_extension(&self, folder_path: &Utf8PathBuf, extension: &str) -> Vec<Utf8PathBuf> {
        let mut files = Vec::new();
        if let Ok(paths) = fs::read_dir(&folder_path) {
            for dir_entry_res in paths {
                if let Ok(dir_entry) = dir_entry_res {
                    if let Ok(path) = Utf8PathBuf::try_from(dir_entry.path()) {
                        if path.extension() == Some(extension) {
                            files.push(path.strip_prefix(&folder_path).unwrap().to_path_buf());
                        }
                    }
                }
            }
        }
        files
    }

    fn get_reader(&self, file_path: &Utf8PathBuf) -> FileSystemResult<Box<dyn Read>> {
        match File::open(file_path) {
            Ok(file) => Ok(Box::new(file)),
            Err(_) => Err(FileSystemError::FileNotFound),
        }
    }

    fn get_writer(&mut self, file_path: &Utf8PathBuf) -> FileSystemResult<Box<dyn Write>> {
        match File::create(file_path) {
            Ok(file) => Ok(Box::new(file)),
            Err(_) => Err(FileSystemError::FileNotFound),
        }
    }
}
