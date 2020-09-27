extern crate repsheet_etl;

use std::collections::HashMap;
use std::collections::hash_map::Entry::{Vacant, Occupied};
use std::hash;

use repsheet_etl::actor::Actor;
use repsheet_etl::response::Response;
use repsheet_etl::address::Address;
use std::time::Instant;
use std::borrow::BorrowMut;

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
    let start = Instant::now();
    let log_entries = repsheet_etl::nginx::process("samples/*");
    let mut actors = repsheet_etl::processor::into_actors(log_entries);
    apply_ruleset(actors.borrow_mut());
    let duration = start.elapsed();

    println!("Processed {} actors in {:?}", actors.len(), duration);
}
