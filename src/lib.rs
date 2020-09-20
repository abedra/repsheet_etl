use std::collections::HashMap;
use std::path::Path;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub mod actor;
mod log_entry;
mod http;
mod hash_utils;

mod nginx;

pub fn process(logfile: &str) -> Result<HashMap<String, actor::Actor>, String> {
    let path = Path::new(logfile);
    let file_handle = match File::open(&path) {
        Err(why) => return Err(format!("Could not open {} : {}", path.display(), why)),
        Ok(file) => file,
    };

    let reader = BufReader::new(file_handle);
    let mut actors = HashMap::<String, actor::Actor>::new();
    for line in reader.lines() {
        match line {
            Ok(line) => nginx::process(&mut actors, &line),
            Err(e)   => return Err(format!("ERROR {}", e)),
        }
    }

    return Ok(actors);
}
