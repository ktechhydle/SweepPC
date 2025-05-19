use crate::framework;
use colored::Colorize;
use std::fs;
use std::io;

pub fn evaluate_results(results: io::Result<Vec<String>>) {
    match results {
        Ok(found_files) => {
            if !found_files.is_empty() {
                let mut displayed_results = String::new();

                for result in &found_files {
                    displayed_results.push_str(&format!(
                        "{} {}\n",
                        "File:".italic().cyan(),
                        result
                    ));
                }

                if !displayed_results.is_empty() {
                    println!(
                        "\n{} {} {} \n\n{}",
                        "SweepPC found".bold().green(),
                        found_files.len().to_string().bold().green(),
                        "result(s)".bold().green(),
                        displayed_results
                    );

                    let delete = framework::get_answer::get_yes_or_no(
                        "Would you like to delete the found files?",
                    );

                    if delete {
                        for file in found_files {
                            let _ = fs::remove_file(file);
                        }
                        println!("{}", "Deleted found files".green());
                    }
                }
            } else {
                println!(
                    "{}",
                    "SweepPC didn't find any files, your computer is clean 😊".white()
                )
            }
        }
        Err(e) => eprintln!("{} {}", "Error scanning directory:".red(), e),
    }
}
