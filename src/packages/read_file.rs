use crate::file_system::{FileSystem, FileSystemError};
use crate::packages::{Package, PackageDependency, PackageSource};
use camino::Utf8PathBuf;
use std::io::Read;
use toml::Table;

#[derive(Debug)]
pub enum PackageError {
    IoError(FileSystemError),
    NonUtf8String,
    TomlError(toml::de::Error),
}

pub fn read_project_toml<T: FileSystem>(
    project_directory: &Utf8PathBuf,
    fs: &T,
) -> Result<Package, PackageError> {
    let project_toml = project_directory.join("project.toml");
    let mut reader = fs
        .get_reader(&project_toml)
        .map_err(|e| PackageError::IoError(e))?;

    let mut toml_string = Default::default();

    reader
        .read_to_string(&mut toml_string)
        .map_err(|e| PackageError::NonUtf8String)?;

    let table: Table = toml::from_str(&toml_string).map_err(|e| PackageError::TomlError(e))?;

    let package_field = table.get("package").unwrap().as_table().unwrap();
    let dependencies_field = table.get("dependencies").unwrap().as_table().unwrap();

    let dependencies: Vec<PackageDependency> = dependencies_field
        .iter()
        .map(|(name, source)| {
            let source = source.as_table().unwrap();
            let source = if source.contains_key("path") {
                let path = source.get("path").unwrap().as_str().unwrap();
                PackageSource::Local(Utf8PathBuf::from(path))
            } else {
                unimplemented!()
            };

            PackageDependency {
                name: name.to_string(),
                source,
            }
        })
        .collect();

    let package = Package {
        name: package_field
            .get("name")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string(),
        stored_location: None,
        root: false,
        dependencies,
    };

    Ok(package)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::file_system::concrete::mock_fs::MockFileSystem;

    #[test]
    fn test_read_project_toml() {
        let mut fs = MockFileSystem::new();
        fs.insert_file(
            Utf8PathBuf::from("test").join("project.toml"),
            r#"
        [package]
        name = "main_pkg"
        version = "0.1.0"

        [dependencies]
        sample_dep = { path = "../sample_dep" }
        "#,
        );

        let package = read_project_toml(&Utf8PathBuf::from("test"), &fs).unwrap();

        assert_eq!(package.name, "main_pkg");
    }
}
