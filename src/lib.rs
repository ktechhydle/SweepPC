use colored::Colorize;
use std::path::PathBuf;
mod framework;

pub fn run_cleanup() {
    println!("{}", "Running default cleanup 🏃‍♂️‍➡️".purple());
    println!(
        "{} {} 👴",
        "Looking for large and old files".green(),
        "(>100 MB and over a year old)".italic().green()
    );
    println!("{} 🗑️", "Looking for temporary files".green());

    let results = framework::scanner::scan_all();

    framework::evaluate::evaluate_results(results)
}

pub fn run_cleanup_on_dir(dir: &String) {
    let path = PathBuf::from(dir);
    let option = Option::from(path);
    let results = framework::scanner::scan_through_files(option);

    framework::evaluate::evaluate_results(results);
}

pub fn run_cleanup_on_temp() {
    println!("{}", "Running temporary cleanup 🏃‍♂️‍➡️".purple());
    let results = framework::scanner::scan_temps();

    framework::evaluate::evaluate_results(results);
}
