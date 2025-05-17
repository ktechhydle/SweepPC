use std::path::PathBuf;
mod framework;

pub fn run_cleanup() {
    println!("Running default cleanup");
    println!("Looking for large and old files (>100 MB and over a year old)");

    let results = framework::scanner::scan_all();

    framework::evaluate::evaluate_results(results)
}

pub fn run_cleanup_on_dir(dir: &String) {
    let path = PathBuf::from(dir);
    let option = Option::from(path);
    let results = framework::scanner::scan_large_and_old_files(option);

    framework::evaluate::evaluate_results(results);
}

pub fn run_cleanup_on_temp() {
    let results = framework::scanner::scan_temps();

    framework::evaluate::evaluate_results(results);
}
