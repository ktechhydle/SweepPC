use colored::Colorize;
use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::time::{Duration, SystemTime};
use walkdir::WalkDir;

pub fn scan_large_and_old_files(dir: Option<PathBuf>) -> io::Result<Vec<String>> {
    let dir_path = match dir {
        Some(p) => p,
        None => {
            eprintln!("{} 😔", "A specified directory was not found".red());
            return Ok(Vec::new());
        }
    };

    let one_year = Duration::from_secs(365 * 24 * 60 * 60); // roughly one year
    let now = SystemTime::now();
    let one_year_ago = now.checked_sub(one_year).expect("Time went backwards");
    let mut entries = WalkDir::new(&dir_path).into_iter();
    let mut results = Vec::new();

    println!(
        "{} {:?} 🔎",
        "Searching".white(),
        &dir_path.to_string_lossy().to_string().replace("\\", "/")
    );

    while let Some(Ok(entry)) = entries.next() {
        let path = entry.path();

        if path.is_file() {
            let metadata = fs::metadata(&path)?;
            let max_size = 100 * 1000 * 1000;

            if metadata.len() > max_size {
                if let Ok(modified_time) = metadata.modified() {
                    if modified_time < one_year_ago {
                        if let Some(path_str) = path.to_str() {
                            results.push(path_str.to_string());
                        }
                    }
                } else {
                    eprintln!(
                        "{} {}",
                        "Could not get modified time for ".red(),
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

pub fn scan_temps() -> io::Result<Vec<String>> {
    let mut results = Vec::new();
    let temp_dir = env::temp_dir();

    println!(
        "{} {:?} 🔎",
        "Searching".white(),
        &temp_dir.to_string_lossy().to_string().replace("\\", "/")
    );

    for entry in WalkDir::new(&temp_dir).into_iter().filter_map(Result::ok) {
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

pub fn scan_all() -> io::Result<Vec<String>> {
    let mut combined_results = Vec::new();

    let sources = [
        scan_large_and_old_files(dirs::desktop_dir()),
        scan_large_and_old_files(dirs::document_dir()),
        scan_large_and_old_files(dirs::download_dir()),
        scan_large_and_old_files(dirs::video_dir()),
        scan_large_and_old_files(dirs::picture_dir()),
        scan_large_and_old_files(dirs::audio_dir()),
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
