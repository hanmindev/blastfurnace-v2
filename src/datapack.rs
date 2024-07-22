use crate::back::commands::{CommandContext, CommandConvertable};
use crate::back::hmasm_types::HmasmFile;
use crate::file_system::FileSystem;
use crate::modules::{module_id_to_path_buf, ModuleId};
use camino::Utf8PathBuf;
use std::fmt::format;
use std::io::Write;

pub struct DatapackWriter<'p, T: FileSystem> {
    root_path: Utf8PathBuf,
    function_root_prefix: String,
    fs: &'p mut T,
}

impl<'p, T: FileSystem> DatapackWriter<'p, T> {
    pub fn new(
        root_path: Utf8PathBuf,
        function_root_prefix: &str,
        fs: &'p mut T,
    ) -> DatapackWriter<'p, T> {
        DatapackWriter {
            root_path: root_path.join(Utf8PathBuf::from("ingot_packages")),
            function_root_prefix: function_root_prefix.to_string(),
            fs,
        }
    }

    fn write(&mut self, root_path: &Utf8PathBuf, extra: &str, content: &str) {
        let mut root_root_path = root_path.clone();
        let mut add = root_root_path.file_name().unwrap().to_string();
        root_root_path.pop();

        let mut writer = self
            .fs
            .get_writer(&root_root_path.join(format!("{add}-{extra}.mcfunction")))
            .unwrap(); // TODO: handle error
        writer.write_all(content.as_bytes()).unwrap(); // TODO: handle error
    }

    pub fn write_file(&mut self, module_id: &ModuleId, content: &HmasmFile) {
        let path = self.root_path.join(module_id_to_path_buf(module_id));
        let mut ctx = CommandContext {
            additional_functions: 0,
        };

        for function in content.functions.iter() {
            let (function_content, additional_functions) = function.to_commands(&mut ctx);
            self.write(&path, &function.name, &function_content);

            for (i, additional_function) in additional_functions.iter().enumerate() {
                self.write(
                    &path,
                    &format!("{}-{}", i, function.name),
                    additional_function,
                );
            }
        }
    }
}
