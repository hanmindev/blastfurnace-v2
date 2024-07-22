mod back;
mod datapack;
mod file_system;
mod front;
mod middle;
mod modules;
mod packages;

#[cfg(test)]
mod tests {
    use crate::datapack::DatapackWriter;
    use crate::file_system::concrete::system_fs::SystemFs;
    use crate::file_system::FileSystem;
    use crate::modules::ModuleBuilder;
    use crate::packages::PackageReader;
    use camino::Utf8PathBuf;
    use std::env;
    #[test]
    fn test_overall_flow() {
        let mut system_fs = SystemFs::new().unwrap();

        let abs_path =
            Utf8PathBuf::from_path_buf(env::current_dir().unwrap().to_path_buf()).unwrap();

        let mut package_reader = PackageReader::new(
            abs_path.join(Utf8PathBuf::from("sample_project\\main_pkg")),
            &system_fs,
        );

        let mut system_fs = SystemFs::new().unwrap();
        let mut module_builder = ModuleBuilder::new(&mut system_fs, None);

        package_reader.add_packages_to_modules(&mut module_builder);

        module_builder.load_module_bodies().unwrap();
        module_builder.build_all_modules();

        let modules = module_builder.get_modules();

        let mut system_fs = SystemFs::new().unwrap();
        let mut datapack_writer = DatapackWriter::new(
            abs_path.join(Utf8PathBuf::from("sample_project\\datapack")),
            "ingot_packages:",
            &mut system_fs,
        );

        for (module_id, module) in modules.iter() {
            datapack_writer.write_file(
                module_id,
                &module.body.as_ref().unwrap().object.as_ref().unwrap(),
            );
        }
    }
}
