use std::{fs, path::PathBuf};

#[derive(Clone)]
pub struct Module {
    pub path: PathBuf,
    pub contents: String,
}

impl Module {
    pub fn new(base: &PathBuf, name: String) -> Module {
        let full_path = Module::get_module_path(&base, &name);
        Module::from_path(full_path)
    }

    pub fn from_path(path: PathBuf) -> Module {
        let contents = fs::read_to_string(&path).unwrap();
        Module { contents, path }
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
        modules
            .iter()
            .find(|m| {
                // println!("has module ? {:?} vs {:?}", m.path, path);
                &m.path == path
            })
            .is_some()
    }
}
