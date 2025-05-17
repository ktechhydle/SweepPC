use std::io;
use crate::framework;

pub fn evaluate_results(results: io::Result<Vec<String>>) {
    match results {
        Ok(found_files) => {
            if !found_files.is_empty() {
                let mut displayed_results = String::new();

                for result in &found_files {
                    displayed_results.push_str(&format!("File: {}\n", result));
                }

                println!(
                    "SweepPC found the following results: \n\n{}",
                    displayed_results
                );

                let delete =
                    framework::get_answer::get_yes_or_no("Would you like to delete the found files?");

                if delete {
                    println!("Deleted found files");
                }
            } else {
                println!("SweepPC didn't find any files, your computer is clean!")
            }
        }
        Err(e) => eprintln!("Error scanning directory: {}", e),
    }
}