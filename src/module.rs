use std::{fs, path::PathBuf};

pub struct Module {
    pub full_path: PathBuf,
    pub contents: String,
}

impl Module {
    pub fn new(base: &PathBuf, name: String) -> Module {
        let full_path = Module::get_module_path(&base, &name);
        Module::from_path(full_path)
    }

    pub fn from_path(full_path: PathBuf) -> Module {
        let contents = fs::read_to_string(&full_path).unwrap();
        Module {
            contents,
            full_path,
        }
    }

    /// Transforms a module name to a full path,
    /// relative to the calling module's path
    pub fn get_module_path(base: &PathBuf, name: &String) -> PathBuf {
        // Make sure that the base path is a folder
        let base = if base.is_file() {
            base.parent().unwrap()
        } else {
            base
        };
        let mut relative_path = name.replace(".", "/");
        relative_path.push_str(".fnl");
        base.join(relative_path)
    }

    pub fn has_module(modules: &Vec<Module>, path: &PathBuf) -> bool {
        modules.iter().find(|m| &m.full_path == path).is_some()
    }
}
