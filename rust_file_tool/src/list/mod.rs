use std::fs;
use std::path::Path;

pub fn list_directory_contents(dir: &Path) {
    match fs::read_dir(dir) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    println!("{}", entry.path().display());
                }
            }
        },
        Err(e) => println!("Error reading directory: {}", e),
    }
}