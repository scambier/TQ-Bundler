use std::{fs, io, path::PathBuf};

use crate::config::Config;

#[derive(Clone, Debug)]
pub struct Module {
    pub file_path: PathBuf,
    pub contents: String,
    pub injected: bool,
}

impl Module {
    pub fn new(file_path: &PathBuf) -> io::Result<Self> {
        let contents = fs::read_to_string(file_path)?;

        Ok(Module {
            file_path: file_path.clone(),
            contents,
            injected: false,
        })
    }

    pub fn has_module(modules: &[Module], file_path: &PathBuf) -> bool {
        modules.iter().any(|m| &m.file_path == file_path)
    }
}

pub fn dotted_to_path(dots: &str, config: &Config) -> PathBuf {
    let mut path = config.base_folder.clone();
    for part in dots.split('.') {
        path.push(part);
    }
    path.set_extension(&config.filetype.extension);
    path
}
