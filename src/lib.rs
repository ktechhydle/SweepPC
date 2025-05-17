use std::path::PathBuf;
mod indexing;
mod input;

pub fn run_cleanup() {
    println!("Running default cleanup");
    println!("Looking for large and old files (>100 MB and over a year old)");

    let results = indexing::scanner::scan_all();

    if !results.is_empty() {
        let mut displayed_results = String::new();

        for result in &results {
            displayed_results.push_str(&format!("File: {}\n", result));
        }

        println!(
            "SweepPC found the following results: \n\n{}",
            displayed_results
        );

        let delete = input::get_answer::get_yes_or_no("Would you like to delete the found files?");

        if delete {
            println!("Deleted found files");
        }
    } else {
        println!("SweepPC didn't find any files, your computer is clean!")
    }
}

pub fn run_cleanup_on_dir(dir: &String) {
    let path = PathBuf::from(dir);
    let option = Option::from(path);
    let results = indexing::scanner::scan_large_and_old_files(option);

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
                    input::get_answer::get_yes_or_no("Would you like to delete the found files?");

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
