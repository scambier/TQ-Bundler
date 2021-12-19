use rust_embed::RustEmbed;
use std::{env, fs, path::Path, process::exit};

use crate::log;

#[derive(RustEmbed)]
#[folder = "src/init/"]
#[prefix = "init/"]
struct Asset;

pub fn initialize(lang: &str) {
    let lang = match lang.to_lowercase().as_str() {
        "lua" => "lua",
        "moonscript" | "moon" => "moon",
        "fennel" | "fnl" => "fnl",
        "squirrel" | "nut" => "nut",
        "wren" => "wren",
        "javascript" | "js" => "js",
        "ruby" | "rb" => "rb",
        _ => {
            log(
                r#"Invalid file type. Use "lua", "moon, "fennel", "squirrel", "wren", "js", "rb"#
                    .to_string(),
            );
            exit(1);
        }
    };

    // Pattern that match the chosen lang
    let pattern = format!("/{:}/", lang);
    let filtered = Asset::iter().filter(|p| p.contains(pattern.as_str()));
    let path = env::current_dir().unwrap();

    // Iterate files corresponding to chosen lang
    for item in filtered.map(|o| o.to_string()) {
        let filepath = Path::new(item.as_str());
        let filename = filepath.file_name().unwrap().to_str().unwrap();
        let file = Asset::get(&item).unwrap();
        let full_path = path.join(filename);

        // Check if the file already exist to not overwrite it
        if full_path.exists() {
            log(format!("! {:} already exists - skipping", filename));
            continue;
        }

        // Write the file
        match fs::write(full_path, file) {
            Ok(_) => {
                log(format!("Created {:}", filename));
            }
            Err(e) => {
                log(e.to_string());
            }
        }
    }
}
