use crate::framework;
use colored::Colorize;
use std::fs;
use std::io::Result;

pub fn evaluate_results(results: Result<Vec<String>>) {
    let found_files = match results {
        Ok(p) => p,
        Err(_) => {
            eprintln!("{}", "A specified directory was not found".red());
            Vec::new()
        }
    };

    if !found_files.is_empty() {
        let mut displayed_results = String::new();

        for result in &found_files {
            displayed_results.push_str(&format!(
                "{} {}\n",
                "File:".italic().cyan(),
                result.to_string().replace("\\", "/")
            ));
        }

        println!(
            "\n{} {} {} \n\n{}",
            "SweepPC found".bold().green(),
            found_files.len().to_string().bold().green(),
            "result(s)".bold().green(),
            displayed_results
        );

        let delete =
            framework::get_answer::get_yes_or_no("Would you like to delete the found files?");

        if delete {
            for file in found_files {
                let _ = fs::remove_file(file);
            }
            println!("\n{}\n", "Deleted found files ✅".green());
        }
    } else {
        println!(
            "\n{}\n",
            "SweepPC didn't find any files, your computer is clean 😊".white()
        )
    }
}
