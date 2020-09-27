extern crate repsheet_etl;

use std::time::Instant;

fn main() {
    let start = Instant::now();
    let log_entries = repsheet_etl::nginx::process("samples/*");
    let actors = repsheet_etl::processor::into_actors(log_entries);
    let duration = start.elapsed();

    println!("Processed {} actors in {:?}", actors.len(), duration);
}
