use std::path::PathBuf;
use std::fs::File;
use std::io::{BufReader, BufRead};

#[allow(dead_code)]
pub fn collect_log_files(pattern: &str) -> Vec<PathBuf> {
    let mut file_paths: Vec<PathBuf> = Vec::new();

    for entry in glob::glob(pattern).expect("Failed to read pattern") {
        match entry {
            Ok(path) => file_paths.push(path),
            Err(e) => println!("{:?}", e)
        }
    }

    return file_paths;
}

#[allow(dead_code)]
pub fn read_log_files(files: Vec<PathBuf>) -> Vec<String> {
    let mut logs: Vec<Vec<String>> = Vec::new();

    for file in files {
        logs.push(read_log_file(file));
    }

    return logs.into_iter().flatten().collect::<Vec<String>>();
}

#[allow(dead_code)]
pub fn read_log_file(file: PathBuf) -> Vec<String> {
    let file_handle = File::open(&file).unwrap();
    let reader = BufReader::new(file_handle);

    let mut log_entries: Vec<String> = Vec::new();
    for line in reader.lines() {
        match line {
            Ok(line) => log_entries.push(line),
            Err(_)   => {},
        }
    }

    return log_entries;
}
