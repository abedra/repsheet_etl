extern crate repsheet_etl;

use std::collections::HashMap;
use std::collections::hash_map::Entry::{Vacant, Occupied};
use std::hash;

use repsheet_etl::actor::Actor;
use repsheet_etl::response::Response;
use repsheet_etl::address::Address;

fn lookup_or_zero<A: Eq + hash::Hash>(hash: &mut HashMap<A, i64>, key: A) -> i64 {
    return match hash.entry(key) {
        Occupied(v) => *v.get(),
        Vacant(_) => 0,
    }
}

fn apply_ruleset(actors: &mut HashMap<Address, Actor>) {
    for (address, actor) in actors {
        if lookup_or_zero(&mut actor.responses, Response::from("404")) > 1 {
            println!("Blacklisting {:?}. Too many 404s", address);
        }
    }
}

fn main() {
    match repsheet_etl::process("samples/access.log") {
        Ok(mut actors) => apply_ruleset(&mut actors),
        Err(e) => println!("{}", e),
    };
}
