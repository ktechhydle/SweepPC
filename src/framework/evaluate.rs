use crate::framework;
use simply_colored::*;
use std::fs;

pub fn evaluate_results(results: Result<Vec<String>, String>) {
    let found_files = match results {
        Ok(files) => files,
        Err(error) => {
            println!("{}", error);

            Vec::new()
        }
    };

    if !found_files.is_empty() {
        let mut displayed_results = String::new();

        for result in &found_files {
            displayed_results.push_str(&format!(
                "📁 File {BOLD}'{}'{RESET}\n",
                result.to_string().replace("\\", "/")
            ));
        }

        println!(
            "🎉 {DIM_GREEN}{BOLD}SweepPC found {} {}{RESET}\n{}",
            found_files.len().to_string(),
            if found_files.len() > 1 {
                "results"
            } else {
                "result"
            },
            displayed_results
        );

        if framework::get_answer::get_yes_or_no("Would you like to delete the found files?") {
            for file in found_files {
                let _ = fs::remove_file(file);
            }

            println!("✅ {DIM_GREEN}Deleted found files{RESET}");
        } else {
            println!("✅ {DIM_GREEN}Process finished, thanks for using!{RESET}");
        }
    } else {
        println!("😊 {DIM_WHITE}SweepPC didn't find any files, your computer is clean{RESET}")
    }
}
