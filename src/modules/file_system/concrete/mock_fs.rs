use camino::Utf8PathBuf;
use std::collections::{HashMap, HashSet};
use crate::modules::file_system::byte_stream::{ByteStream, StringReader};
use crate::modules::file_system::{FileSystem, FileSystemError, FileSystemResult};

pub struct MockFileSystem {
    files: HashMap<Utf8PathBuf, String>,
    dirs: HashSet<Utf8PathBuf>,
}

impl MockFileSystem {
    pub fn new() -> MockFileSystem {
        MockFileSystem {
            files: Default::default(),
            dirs: Default::default(),
        }
    }
    pub fn insert_file(&mut self, path: Utf8PathBuf, content: &str) {
        self.files.insert(path, content.to_string());
    }

    pub fn insert_dir(&mut self, path: Utf8PathBuf) {
        self.dirs.insert(path);
    }
}


impl FileSystem for MockFileSystem {
    fn list_files_with_extension(&self, path: &Utf8PathBuf, extension: &str) -> Vec<Utf8PathBuf> {
        let mut files = Vec::new();
        for (file_path, _) in self.files.iter() {
            match file_path.strip_prefix(&path) {
                Ok(stripped_path) => {
                    if stripped_path.extension() == Some(extension)
                        && stripped_path.parent() == Some(&Utf8PathBuf::new())
                    {
                        files.push(file_path.clone());
                    }
                }
                Err(_) => {
                    continue;
                }
            }
        }
        files
    }

    fn read_file(&self, file_path: &Utf8PathBuf) -> FileSystemResult<ByteStream> {
        match self.files.get(file_path) {
            Some(content) => Ok(ByteStream::new(Box::new(StringReader::new(
                content.to_string(),
            )))),
            None => Err(FileSystemError::FileNotFound),
        }
    }
}