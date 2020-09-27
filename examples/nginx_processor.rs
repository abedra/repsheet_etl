extern crate repsheet_etl;

use std::time::Instant;

fn main() {
    let start = Instant::now();
    let results = repsheet_etl::nginx::process("samples/*");
    let valid_count = results.valid.len();
    let invalid_count = results.invalid.len();
    let duration = start.elapsed();

    println!("Processed {} valid and {} invalid log entries in {:?}", valid_count, invalid_count, duration);
}