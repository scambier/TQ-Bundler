use std::path::{Path, PathBuf};

use clap::ArgMatches;

pub struct Config {
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
        let str_path = matches.value_of("CODE").unwrap();
        let file_path = Path::new(str_path);
        if !file_path.is_file() {
            panic!("{:?} is not a valid file", &file_path);
        }
        let file = file_path.file_stem().unwrap().to_str().unwrap();
        let base_folder = file_path.parent().unwrap();

        let tic_path = match matches.value_of("TIC") {
            Some(v) => Some(v.to_string()),
            None => None,
        };

        Config {
            game: String::from(matches.value_of("GAME").unwrap()),
            entry_point: String::from(file),
            tic_path,
            base_folder: base_folder.to_path_buf(),
            output_file: matches
                .value_of("OUTPUT")
                .unwrap_or("build.fnl")
                .to_string(),
            watch: matches.is_present("WATCH"),
        }
    }
}
