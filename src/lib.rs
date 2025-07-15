use simply_colored::*;
use std::path::PathBuf;

mod framework;

pub fn run_cleanup() {
    println!("⏩ Running default cleanup");
    println!("🕵️  Scanning for large & old files {ITALIC}(>100 MB, older than a year)");
    println!("🗑️  Scanning for temporary files");

    let results = framework::scanner::scan_all();
    framework::evaluate::evaluate_results(results);
}

pub fn run_cleanup_on_dir(dir: &str) {
    println!("📁 Running cleanup on: {BOLD}'{dir}'{RESET}");

    let path = PathBuf::from(dir);

    if !path.exists() {
        println!("⚠️  Warning: {DIM_RED}provided path doesn't exist{RESET}");

        return;
    }

    let results = framework::scanner::scan_through_files(Some(path));

    framework::evaluate::evaluate_results(results);
}

pub fn run_cleanup_on_temp() {
    println!("⏩ Running temporary files cleanup");

    let results = framework::scanner::scan_temps();
    framework::evaluate::evaluate_results(results);
}
