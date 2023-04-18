use std::{fs, io::Error, path::PathBuf};

use crate::{config::Config, log};

#[derive(Clone, Debug)]
pub struct Module {
    pub file_path: PathBuf,
    pub contents: String,
}

impl Module {
    pub fn new(file_path: &PathBuf, config: &Config) -> Self {
        assert!(
            file_path.is_file(),
            "file_path {:?} is not a file",
            file_path
        );
        let folder = file_path.parent().unwrap();
        let mut contents = match fs::read_to_string(&file_path) {
            Ok(contents) => contents,
            Err(e) => panic!("Error reading file: {}", e),
        };

        // Rewrite includes to be relative to the root folder
        let dotted_folder = path_to_dotted(&folder.to_path_buf());
        println!("path: {:?}", folder);
        println!("dotted: {:?}", dotted_folder);
        if dotted_folder.len() > 0 {
            let reg_include = &config.filetype.regex;
            for capture in reg_include.captures_iter(&contents.clone()) {
                println!("capture: {:?}", capture);
                let cap = capture.get(1).unwrap();
                let range = cap.range();
                let to_replace = &format!("{dotted_folder}.{:}", cap.as_str());
                contents.replace_range(range, to_replace);
                println!("contents: {:?}", contents);
                println!("^^^^^^^^");
            }
        }

        let module = Module {
            file_path: file_path.clone(),
            contents,
        };
        println!("module: {:?}", module);
        println!("--");
        module
    }

    pub fn has_module(modules: &Vec<Module>, file_path: &PathBuf) -> bool {
        modules
            .iter()
            .find(|m| {
                // println!("has module ? {:?} vs {:?}", m.path, path);
                &m.file_path == file_path
            })
            .is_some()
    }
}

fn path_to_dotted(path: &PathBuf) -> String {
    let mut parts = path.iter().map(|p| p.to_string_lossy()).collect::<Vec<_>>();
    if parts.len() == 0 {
        return "".to_string();
    }
    parts.remove(0);
    let dotted_folder = parts.join(".");
    dotted_folder
}

pub fn dotted_to_path(dots: &str, config: &Config) -> PathBuf {
    let mut path = config.base_folder.clone();
    for part in dots.split('.') {
        path.push(part);
    }
    path.set_extension(&config.filetype.extension);
    path
}

// impl Module {
//     pub fn new(base: &PathBuf, name: &String, ext: &String) -> Result<Module, Error> {
//         let full_path = Module::get_module_path(&base, &name, ext);
//         Module::from_path(full_path)
//     }

//     pub fn from_path(path: PathBuf) -> Result<Module, Error> {
//         log(format!("Reading {:?}", path));
//         let contents = fs::read_to_string(&path);
//         match contents {
//             Ok(contents) => Ok(Module { contents, path }),
//             Err(e) => Err(e),
//         }
//     }

//     /// Transforms a module name to a full path,
//     /// relative to the main entry point
//     pub fn get_module_path(base: &PathBuf, name: &String, ext: &String) -> PathBuf {
//         // Make sure that the base path is a folder
//         let base = if base.is_file() {
//             base.parent().unwrap()
//         } else {
//             base
//         };
//         // Convert the module name into a path
//         let mut relative_path = PathBuf::new();
//         let mut split = name.split(".").collect::<Vec<_>>();
//         // Append the extension to the last part of the path (the filename)
//         let filename_with_ext = format!("{}.{}", split.last().unwrap(), ext);
//         split.pop();
//         split.push(&filename_with_ext);

//         for part in split {
//             relative_path.push(part);
//         }
//         base.join(relative_path)
//     }

//     pub fn has_module(modules: &Vec<Module>, path: &PathBuf) -> bool {
//         modules
//             .iter()
//             .find(|m| {
//                 // println!("has module ? {:?} vs {:?}", m.path, path);
//                 &m.path == path
//             })
//             .is_some()
//     }
// }
