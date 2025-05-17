use crate::framework;
use std::fs;
use std::io;
use std::path::Path;

pub fn evaluate_results(results: io::Result<Vec<String>>) {
    match results {
        Ok(found_files) => {
            if !found_files.is_empty() {
                let mut displayed_results = String::new();

                for result in &found_files {
                    let path = Path::new(result);
                    let file_name = path.file_name();

                    match file_name.unwrap().to_str() {
                        Some(basename) => {
                            displayed_results.push_str(&format!("File: {}\n", basename.to_string()))
                        }
                        None => {}
                    }
                }

                if !displayed_results.is_empty() {
                    println!(
                        "SweepPC found the following results: \n\n{}",
                        displayed_results
                    );

                    let delete = framework::get_answer::get_yes_or_no(
                        "Would you like to delete the found files?",
                    );

                    if delete {
                        for file in found_files {
                            let _ = fs::remove_file(file);
                        }
                        println!("Deleted found files");
                    }
                }
            } else {
                println!("SweepPC didn't find any files, your computer is clean!")
            }
        }
        Err(e) => eprintln!("Error scanning directory: {}", e),
    }
}
