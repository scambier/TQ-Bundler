use std::{fs, io::Error, path::PathBuf};

use crate::log;

#[derive(Clone)]
pub struct Module {
    pub path: PathBuf,
    pub contents: String,
}

impl Module {
    pub fn new(base: &PathBuf, name: &String, ext: &String) -> Result<Module, Error> {
        let full_path = Module::get_module_path(&base, &name, ext);
        Module::from_path(full_path)
    }

    pub fn from_path(path: PathBuf) -> Result<Module, Error> {
        log(format!("Reading {:?}", path));
        let contents = fs::read_to_string(&path);
        match contents {
            Ok(contents) => Ok(Module { contents, path }),
            Err(e) => Err(e),
        }
    }

    /// Transforms a module name to a full path,
    /// relative to the main entry point
    pub fn get_module_path(base: &PathBuf, name: &String, ext: &String) -> PathBuf {
        // Make sure that the base path is a folder
        let base = if base.is_file() {
            base.parent().unwrap()
        } else {
            base
        };
        // Convert the module name into a path
        let mut relative_path = PathBuf::new();
        let mut split = name.split(".").collect::<Vec<_>>();
        // Append the extension to the last part of the path (the filename)
        let filename_with_ext = format!("{}.{}", split.last().unwrap(), ext);
        split.pop();
        split.push(&filename_with_ext);

        for part in split {
            relative_path.push(part);
        }
        base.join(relative_path)
    }

    pub fn has_module(modules: &Vec<Module>, path: &PathBuf) -> bool {
        modules
            .iter()
            .find(|m| {
                // println!("has module ? {:?} vs {:?}", m.path, path);
                &m.path == path
            })
            .is_some()
    }
}
