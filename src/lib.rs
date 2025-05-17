mod indexing;
mod input;

pub fn run_cleanup() {
    println!("Running default cleanup...");

    let results = indexing::scanner::scan_large_and_old_files();
    let mut displayed_results = String::new();

    for filenames in &results {
        for result in filenames {
            displayed_results.push_str(&format!("File: {}\n", result));
        }
    }

    if &displayed_results {
        println!(
            "sweeppc found the following results (files > 100 MB and over a year old): \n\n{}",
            displayed_results
        );

        let delete = input::get_answer::get_yes_or_no("Would you like to delete the found files?");

        if delete {
            println!("Deleting found files...")
        }
    }
}
