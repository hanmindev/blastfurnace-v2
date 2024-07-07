use crate::file_system::{FileSystem, FileSystemError, FileSystemResult};
use camino::Utf8PathBuf;
use std::collections::{HashMap, HashSet};
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};

type Content = String;
struct StringWriter {
    files: Files,
    content: Content,
    file_path: Utf8PathBuf,
}

struct StringReader {
    content: Content,
    position: usize,
}

impl Read for StringReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let bytes = self.content.as_bytes();
        let bytes_to_copy = std::cmp::min(buf.len(), bytes.len() - self.position);
        buf[..bytes_to_copy].copy_from_slice(&bytes[self.position..self.position + bytes_to_copy]);
        self.position += bytes_to_copy;
        Ok(bytes_to_copy)
    }
}

impl Write for StringWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.content.push_str(&String::from_utf8_lossy(buf));

        self.files
            .lock()
            .unwrap()
            .insert(self.file_path.clone(), self.content.clone());

        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

type Files = Arc<Mutex<HashMap<Utf8PathBuf, String>>>;

pub struct MockFileSystem {
    files: Files,
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
        self.files.lock().unwrap().insert(path, content.to_string());
    }

    pub fn insert_dir(&mut self, path: Utf8PathBuf) {
        self.dirs.insert(path);
    }
}

impl FileSystem for MockFileSystem {
    fn list_files_with_extension(&self, path: &Utf8PathBuf, extension: &str) -> Vec<Utf8PathBuf> {
        let mut files = Vec::new();
        for (file_path, _) in self.files.lock().unwrap().iter() {
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

    fn get_reader(&self, file_path: &Utf8PathBuf) -> FileSystemResult<Box<dyn Read>> {
        match self.files.lock().unwrap().get(file_path) {
            Some(content) => Ok(Box::new(StringReader {
                content: content.clone(),
                position: 0,
            })),
            None => Err(FileSystemError::FileNotFound),
        }
    }

    fn get_writer(&mut self, file_path: &Utf8PathBuf) -> FileSystemResult<Box<dyn Write>> {
        Ok(Box::new(StringWriter {
            files: self.files.clone(),
            content: String::new(),
            file_path: file_path.clone(),
        }))
    }

    fn get_file_age(&self, _file_path: &Utf8PathBuf) -> FileSystemResult<u128> {
        return Ok(1);
    }
}
