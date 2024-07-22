mod read_file;

use crate::file_system::FileSystem;
use crate::front::ast_types::PackageName;
use crate::modules::ModuleBuilder;
use crate::packages::read_file::read_project_toml;
use camino::Utf8PathBuf;
use serde::Deserialize;
use std::cmp::PartialEq;
use std::collections::{HashMap, VecDeque};

struct Version(i32, i32, i32);

#[derive(Hash, Eq, Clone, PartialEq)]
enum PackageSource {
    Local(Utf8PathBuf),
    // Remote(String, Version), // TODO: not yet supported
}

#[derive(Hash, Eq, Clone, PartialEq)]
struct PackageDependency {
    name: PackageName,
    source: PackageSource,
}

struct Package {
    name: PackageName,
    stored_location: Option<Utf8PathBuf>,
    root: bool,

    dependencies: Vec<PackageDependency>,
}

pub struct PackageReader<'p, T: FileSystem> {
    root_package: PackageName,
    packages: HashMap<PackageName, Package>,

    dequeue: VecDeque<(PackageDependency, Utf8PathBuf)>,
    fs: &'p T,
}

impl<'p, T: FileSystem> PackageReader<'p, T> {
    pub fn new(root_package_location: Utf8PathBuf, fs: &T) -> PackageReader<T> {
        let mut package = read_project_toml(&root_package_location, fs).unwrap();
        package.root = true;

        let mut pr = PackageReader {
            root_package: package.name.clone(),
            packages: HashMap::new(),
            dequeue: VecDeque::new(),
            fs,
        };
        pr.dequeue.push_back((
            PackageDependency {
                name: package.name.clone(),
                source: PackageSource::Local(root_package_location.clone()),
            },
            root_package_location.clone(),
        ));

        while let Some((package, root_package_location)) = pr.dequeue.pop_front() {
            pr.read_package(&package, &root_package_location);
        }

        pr
    }

    fn read_package(&mut self, package_dependency: &PackageDependency, current_path: &Utf8PathBuf) {
        let (package, package_path) = match &package_dependency.source {
            PackageSource::Local(path) => {
                let package_path = if path.is_absolute() {
                    path.clone()
                } else {
                    current_path.join(path)
                };
                let mut package = read_project_toml(&package_path, self.fs).unwrap();
                package.stored_location = Some(package_path.clone());
                (package, package_path)
            }
            _ => unimplemented!(),
        };

        if self
            .packages
            .insert(package_dependency.name.clone(), package)
            .is_some()
        {
            panic!("Duplicate package name: {}", package_dependency.name); // TODO: properly handle in the future
        }
        let package = self.packages.get(&package_dependency.name).unwrap();

        for dep in &package.dependencies {
            self.dequeue.push_back((dep.clone(), package_path.clone()));
        }
    }

    pub fn add_packages_to_modules(&mut self, module_builder: &mut ModuleBuilder<T>) {
        for (package_name, package) in &mut self.packages {
            let a = package_name.as_str();
            let b = self.root_package.as_str();

            println!("stored {:?}", package.stored_location.as_ref().unwrap());

            // TODO: add proper error handling
            module_builder
                .add_fs_package(
                    package_name,
                    &package.stored_location.as_ref().unwrap(),
                    a == b,
                )
                .unwrap();
        }
    }
}
