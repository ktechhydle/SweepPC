use simply_colored::*;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};
use walkdir::WalkDir;

pub fn scan_through_files(dir: Option<PathBuf>) -> Result<Vec<String>, String> {
    let dir_path = match dir {
        Some(p) => p,
        None => {
            return Err(
                "⚠️ Warning: {DIM_RED}a specified directory was not found{RESET}".to_string(),
            );
        }
    };

    let one_year = Duration::from_secs(365 * 24 * 60 * 60); // roughly one year
    let now = SystemTime::now();
    let one_year_ago = now
        .checked_sub(one_year)
        .expect("⚠️ Warning: {DIM_RED}time somehow went backwards?{RESET}");
    let mut entries = WalkDir::new(&dir_path).into_iter();
    let mut results = Vec::new();

    println!(
        "🔎 Searching {BOLD}'{}'{RESET}",
        &dir_path.to_string_lossy().to_string().replace("\\", "/")
    );

    while let Some(Ok(entry)) = entries.next() {
        let path = entry.path();

        if path.is_file() {
            let metadata = fs::metadata(&path)
                .expect("⚠️ Warning: {DIM_RED}file metadata couldn't be read{RESET}");
            let max_size = 100 * 1000 * 1000;

            if metadata.len() > max_size {
                if let Ok(modified_time) = metadata.modified() {
                    if modified_time < one_year_ago {
                        if let Some(path_str) = path.to_str() {
                            results.push(path_str.to_string());
                        }
                    }
                } else {
                    println!(
                        "⚠️ Warning: {DIM_RED}Could not get modified time for{RESET} {}",
                        path.to_string_lossy().to_string().replace("\\", "/")
                    );
                }
            }
        } else {
            continue;
        }
    }

    Ok(results)
}

pub fn scan_temps() -> Result<Vec<String>, String> {
    let temp_dir = env::temp_dir();
    let mut results = Vec::new();
    let mut entries = WalkDir::new(&temp_dir).into_iter();

    println!(
        "🔎 Searching {BOLD}'{}'{RESET}",
        &temp_dir.to_string_lossy().to_string().replace("\\", "/")
    );

    while let Some(Ok(entry)) = entries.next() {
        let path = entry.path();

        if path.is_file() {
            if let Some(file_name) = path.to_str() {
                results.push(file_name.to_string());
            }
        } else {
            continue;
        }
    }

    Ok(results)
}

pub fn scan_all() -> Result<Vec<String>, String> {
    let mut combined_results = Vec::new();

    let sources = [
        scan_through_files(dirs::desktop_dir()),
        scan_through_files(dirs::document_dir()),
        scan_through_files(dirs::download_dir()),
        scan_through_files(dirs::video_dir()),
        scan_through_files(dirs::picture_dir()),
        scan_through_files(dirs::audio_dir()),
        scan_temps(),
    ];

    for result in sources {
        match result {
            Ok(mut files) => combined_results.append(&mut files),
            Err(e) => return Err(e),
        }
    }

    Ok(combined_results)
}
