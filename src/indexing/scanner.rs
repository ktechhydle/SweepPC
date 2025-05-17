use std::fs;
use std::io;
use walkdir::WalkDir;

pub fn scan_large_and_old_files(dir: &str) -> io::Result<Vec<String>> {
    let mut results = Vec::new();

    for entry in WalkDir::new(dir).into_iter().filter_map(Result::ok) {
        let path = entry.path();

        if path.is_file() {
            let file_name = match path.to_str() {
                Some(name) => name.to_string(),
                None => continue, // skip paths that aren't valid UTF-8
            };

            let metadata = fs::metadata(path)?;
            let length = metadata.len();
            let max_size = 100 * 1000 * 1000; // 100mb

            if length > max_size {
                results.push(file_name);
            }
        }
    }

    Ok(results)
}
