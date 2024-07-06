mod byte_stream;
mod concrete;

use camino::Utf8PathBuf;
use crate::modules::file_system::byte_stream::ByteStream;

#[derive(Debug)]
pub enum FileSystemError {
    FileNotFound,
    DirectoryNotFound,
}

pub type FileSystemResult<T> = Result<T, FileSystemError>;

pub trait FileSystem {
    fn list_files_with_extension(&self, path: &Utf8PathBuf, extension: &str) -> Vec<Utf8PathBuf>;
    fn read_file(&self, file_path: &Utf8PathBuf) -> FileSystemResult<ByteStream>;
}
