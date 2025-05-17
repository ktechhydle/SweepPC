use std::fs;
use std::io;
use std::path::PathBuf;
use walkdir::WalkDir;

pub fn scan_large_and_old_files(dir: Option<PathBuf>) -> io::Result<Vec<String>> {
    let mut results = Vec::new();

    match dir {
        Some(home_dir) => {
            println!("Searching {:?}", &home_dir);

            for entry in WalkDir::new(&home_dir) {
                let path = entry?.path().to_path_buf();
                let metadata = fs::metadata(&path)?;
                let max_size = 100 * 1000 * 1000;

                if metadata.len() > max_size {
                    if let Some(path_str) = path.to_str() {
                        results.push(path_str.to_string());
                    }
                }
            }
        }
        None => {
            eprintln!("I couldn't find the specified directory to search :(")
        }
    }

    Ok(results)
}

pub fn scan_all() -> Vec<String> {
    let mut combined_results = Vec::new();

    let sources = [
        scan_large_and_old_files(dirs::desktop_dir()),
        scan_large_and_old_files(dirs::document_dir()),
        scan_large_and_old_files(dirs::download_dir()),
        scan_large_and_old_files(dirs::video_dir()),
        scan_large_and_old_files(dirs::picture_dir()),
        scan_large_and_old_files(dirs::audio_dir()),
    ];

    for result in sources {
        match result {
            Ok(mut vec) => combined_results.append(&mut vec),
            Err(e) => eprintln!("Error scanning directory: {}", e),
        }
    }

    combined_results
}
