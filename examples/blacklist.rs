extern crate repsheet_etl;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Vacant, Occupied};

fn lookup_or_zero(hash: &mut HashMap<String, i64>, key: &str) -> i64 {
    match hash.entry(key.to_string()) {
        Occupied(v) => return *v.get(),
        Vacant(_) => return 0,
    }
}

fn apply_ruleset(actors: &mut HashMap<String, repsheet_etl::Actor>) {
    for (address, actor) in actors {
        if lookup_or_zero(&mut actor.responses, "404") > 50 {
            println!("Blacklisting {}. Too many 404s", address);
        }
    }
}

fn main() {
    let _ = match repsheet_etl::process("access.log") {
        Ok(mut actors) => { apply_ruleset(&mut actors) },
        Err(e) => println!("{}", e),
    };
}
