use std::fs::File;
use std::io::Error;
use std::path::Path;

pub fn create_file(file_path: &Path) -> Result<(), Error>{
    // Your task: Implement file creation and print success or error message
    match File::create(file_path) {
        Ok(_) => {
            println!("File created successfully: {}", file_path.display());
            Ok(())
        }
        Err(e) => {
            println!("Error creating file: {}", e);
            Err(e)
        }
    }

}