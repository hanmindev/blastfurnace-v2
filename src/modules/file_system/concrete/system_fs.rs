use std::fs;
use std::fs::File;
use std::io::Read;
use crate::modules::file_system::{Utf8PathBuf, FileSystem, FileSystemError, FileSystemResult};
use crate::modules::file_system::byte_stream::{ByteStream, ByteStreamable};

pub struct SystemFs;

pub struct FileReader {
    // TODO: make this more efficient instead of reading the whole file into memory
    file: File,
    str: String,
    index: usize,
}

impl FileReader {
    pub fn new(file: File) -> FileReader {
        let mut file_reader = FileReader {
            file,
            str: String::new(),
            index: 0,
        };
        file_reader
            .file
            .read_to_string(&mut file_reader.str)
            .expect("Unable to read file");
        file_reader
    }
}

impl ByteStreamable for FileReader {
    fn next(&mut self) -> char {
        if self.index >= self.str.len() {
            return '\0';
        }

        let c = self.str.as_bytes()[self.index] as char;
        self.index += 1;
        c
    }
}

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

    fn read_file(&self, file_path: &Utf8PathBuf) -> FileSystemResult<ByteStream> {
        match File::open(file_path) {
            Ok(file) => Ok(ByteStream::new(Box::new(FileReader::new(file)))),
            Err(_) => Err(FileSystemError::FileNotFound),
        }
    }
}
