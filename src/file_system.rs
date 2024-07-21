pub mod concrete;

use camino::Utf8PathBuf;
use std::io::{Read, Write};

#[derive(Debug)]
pub enum FileSystemError {
    FileNotFound,
    DirectoryNotFound,
}

pub type FileSystemResult<T> = Result<T, FileSystemError>;

pub trait FileSystem {
    fn list_files_with_extension(&self, path: &Utf8PathBuf, extension: &str) -> Vec<Utf8PathBuf>;
    fn get_reader(&self, file_path: &Utf8PathBuf) -> FileSystemResult<Box<dyn Read>>;
    fn get_file_age(&self, file_path: &Utf8PathBuf) -> FileSystemResult<u128>;
    fn get_writer(&mut self, file_path: &Utf8PathBuf) -> FileSystemResult<Box<dyn Write>>;
}
