use dirs;
use std::fs;
use std::io;
use walkdir::WalkDir;

pub fn scan_large_and_old_files() -> io::Result<Vec<String>> {
    let mut results = Vec::new();
    let user_dir = dirs::home_dir();

    match user_dir {
        Some(home_dir) => {
            println!("Searching {:?}", &home_dir);

            for entry in WalkDir::new(&home_dir) {
                let path = entry?.path().to_path_buf();
                let metadata = path.metadata()?;
                let max_size = 100 * 1000 * 1000;

                if metadata.len() > max_size {
                    if let Some(path_str) = path.to_str() {
                        results.push(path_str.to_string());
                    }
                }
            }
        }
        None => {
            println!("I couldn't find a user directory to search :(")
        }
    }

    Ok(results)
}
