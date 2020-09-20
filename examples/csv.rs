extern crate repsheet_etl;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Vacant, Occupied};

fn extract_or_zero(map: &mut HashMap<String, i64>, key: &str) -> i64 {
    return match map.entry(key.to_string()) {
        Occupied(e) => *e.get(),
        Vacant(_) => 0,
    }
}

fn write_csv(actors: &mut HashMap<String, repsheet_etl::actor::Actor>) {
    let mut f = File::create("out.csv").unwrap();
    let _ = f.write_all("address, number_of_requests, invalid_requests, GET, POST, PUT, DELETE, HEAD, OPTIONS, TRACE, CONNECT\n".as_bytes());
    for (address, actor) in actors {
        let gets    = extract_or_zero(&mut actor.methods, "GET");
        let posts   = extract_or_zero(&mut actor.methods, "POST");
        let puts    = extract_or_zero(&mut actor.methods, "PUT");
        let deletes = extract_or_zero(&mut actor.methods, "DELETE");
        let heads   = extract_or_zero(&mut actor.methods, "HEAD");
        let options = extract_or_zero(&mut actor.methods, "OPTIONS");
        let traces   = extract_or_zero(&mut actor.methods, "TRACE");
        let connects = extract_or_zero(&mut actor.methods, "CONNECT");
        let _ = f.write_all(format!("{}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}\n",
                                    address,
                                    actor.request_count,
                                    actor.invalid_request_count,
                                    gets,
                                    posts,
                                    puts,
                                    deletes,
                                    heads,
                                    options,
                                    traces,
                                    connects
        ).as_bytes());
    }
}

fn main() {
    match repsheet_etl::process("access.log") {
        Ok(mut actors) => write_csv(&mut actors),
        Err(e) => println!("{}", e),
    };
}
