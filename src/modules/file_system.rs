mod concrete;

use std::io::{Read, Write};
use camino::Utf8PathBuf;

#[derive(Debug)]
pub enum FileSystemError {
    FileNotFound,
    DirectoryNotFound,
}

pub type FileSystemResult<T> = Result<T, FileSystemError>;

pub trait FileSystem {
    fn list_files_with_extension(&self, path: &Utf8PathBuf, extension: &str) -> Vec<Utf8PathBuf>;
    fn get_reader(&self, file_path: &Utf8PathBuf) -> FileSystemResult<Box<dyn Read>>;
    fn get_writer(&mut self, file_path: &Utf8PathBuf) -> FileSystemResult<Box<dyn Write>>;
}
