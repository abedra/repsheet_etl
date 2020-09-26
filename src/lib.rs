extern crate core;
extern crate itertools;
extern crate glob;
extern crate either;

use log_entry::LogEntry;
use invalid_log_entry::InvalidLogEntry;
use file::{collect_log_files, read_log_files};
use either::Either;

pub mod actor;
pub mod method;
pub mod response;
pub mod address;

mod log_entry;
mod hash_utils;
mod nginx;
mod invalid_log_entry;
mod file;

// pub fn process(logfile: &str) -> Result<HashMap<Address, actor::Actor>, String> {
//     let path = Path::new(logfile);
//     let file_handle = match File::open(&path) {
//         Err(why) => return Err(format!("Could not open {} : {}", path.display(), why)),
//         Ok(file) => file,
//     };
//
//     let reader = BufReader::new(file_handle);
//     let mut actors = HashMap::<Address, actor::Actor>::new();
//     for line in reader.lines() {
//         match line {
//             Ok(line) => nginx::process(&mut actors, &line),
//             Err(e)   => return Err(format!("ERROR {}", e)),
//         }
//     }
//
//     return Ok(actors);
// }

pub struct NginxProcessingResults {
    valid: Vec<LogEntry>,
    invalid: Vec<InvalidLogEntry>,
}

pub fn process(pattern: &str) -> NginxProcessingResults {
    let log_entries = read_log_files(collect_log_files(pattern));
    let mut valid_entries: Vec<LogEntry> = Vec::new();
    let mut invalid_entries: Vec<InvalidLogEntry> = Vec::new();

    for entry in log_entries {
        match nginx::process_line(entry.as_str()) {
            Either::Left(result) => invalid_entries.push(result),
            Either::Right(result) => valid_entries.push(result)
        }
    }

    NginxProcessingResults{valid: valid_entries, invalid: invalid_entries}
}