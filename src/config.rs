use std::{
    path::{Path, PathBuf},
    process::exit,
};

use clap::ArgMatches;
use regex::Regex;

use crate::log;

#[derive(Debug)]
pub struct FileType {
    pub extension: String,
    pub regex: Regex,
    pub comment: String,
}

impl FileType {
    pub fn new(filename: &Path) -> FileType {
        // (\n|[\r\n]+) is a fix for the EOL symbol ($) not working on Windows CRLF

        // Regex for `include "my.module"`
        let regex = Regex::new(r#"(?m)^include "([a-zA-Z\-_\.]+)"(\n|[\r\n]+)"#).unwrap();

        let ext = filename.extension().unwrap().to_str().unwrap();
        let extension = ext.to_string();
        match ext {
            "lua" | "moon" => FileType {
                extension,
                regex,
                comment: "--".to_string(),
            },
            "fnl" => FileType {
                extension,
                // Regex for `(include "my.module")`
                regex: Regex::new(r#"(?m)^\(include "([a-zA-Z\-_\.]+)"\)(\n|[\r\n]+)"#).unwrap(),
                comment: ";;".to_string(),
            },
            "wren" => FileType {
                extension,
                regex,
                comment: "//".to_string(),
            },
            "nut" | "js" => FileType {
                extension,
                // Regex for `include("my.module")`
                regex: Regex::new(r#"(?m)^include\("([a-zA-Z\-_\.]+)"\)(\n|[\r\n]+)"#).unwrap(),
                comment: "//".to_string(),
            },
            _ => {
                log(format!(
                    "Supported extensions are .lua, .moon, .fnl, .wren, .nut, .js"
                ));
                exit(1);
            }
        }
    }
}

#[derive(Debug)]
pub struct Config {
    pub filetype: FileType,
    pub base_folder: PathBuf,
    pub game: String,
    pub entry_point: String,
    pub tic_path: Option<String>,
    pub output_file: String,
    pub watch: bool,
}

impl Config {
    /// Creates a new Config instance from clap matches
    pub fn new(matches: &ArgMatches) -> Config {
        // Code entry point
        let code_str_path = matches.value_of("CODE").unwrap();
        let code_file_path = Path::new(code_str_path);
        if !code_file_path.is_file() {
            log(format!("{:?} is not a valid file", &code_file_path));
            exit(1);
        }
        let file = code_file_path.file_stem().unwrap().to_str().unwrap();

        // Reference path for includes
        let base_folder = code_file_path.parent().unwrap().to_path_buf();

        // Optional path to TIC-80; will launch it if present
        let tic_path = match matches.value_of("TIC") {
            Some(v) => Some(v.to_string()),
            None => None,
        };

        // Determine the regex and file extension
        let filetype = FileType::new(&code_file_path);

        Config {
            game: String::from(matches.value_of("GAME").unwrap()),
            entry_point: String::from(file),
            tic_path,
            base_folder,
            output_file: matches
                .value_of("OUTPUT")
                .unwrap_or(format!("build.{:}", &filetype.extension).as_str())
                .to_string(),
            filetype,
            watch: matches.is_present("WATCH"),
        }
    }
}
