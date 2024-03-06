use std::env;
use std::fs;
use std::path::Path;

mod list;
use list::*;

mod create_file;
use create_file::*;

fn main() {
    

    
    let args: Vec<String> = env::args().collect(); // get the args from command line
    if args.len() < 2 {
        // when input args less than 2
        println!("Usage: <command> [arguments]"); 
        return;
    }

    // Placeholder for matching commands
    match args.get(1).map(String::as_str) {
    
        Some("ls") => {
            if args.len() > 2 {
                let path_arg = Path::new(&args[2]);
                // if there is a given path/DIR, use that one
                list_directory_contents(&path_arg);
            } else {
                let path_arg = env::current_dir().expect("Failed to get current directory");
                list_directory_contents(&path_arg); // use the current dir
            }
            {}

        },
        Some("touch") => {
            if let Some(file_name) = args.get(2) {
                let file_path = Path::new(file_name);
                match create_file(&file_path) {
                    Ok(_) => println!("File created successfully."),
                    Err(e) => println!("Failed to create file: {}", e),
                }
            } else {
                println!("Please provide a file name to create.");
            }
        },
        Some("exit") => {
            return;
        },
        _ => println!("Invalid command or not implemented"),
    }

    
}
