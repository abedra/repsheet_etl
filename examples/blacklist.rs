extern crate repsheet_etl;
extern crate redis;

use std::borrow::{BorrowMut, Borrow};
use std::time::Instant;
use std::collections::HashMap;

use repsheet_etl::actor::Actor;
use repsheet_etl::response::Response;
use repsheet_etl::address::Address;
use redis::Client;

fn apply_ruleset(actors: &mut HashMap<Address, Actor>, client: &Client) {
    for (address, actor) in actors {
        let not_found_count = *actor.responses
            .get(Response::from("404").borrow())
            .unwrap_or(&0i64);

        if not_found_count > 10 {
            let mut connection = client.get_connection().unwrap();
            redis::cmd("REPSHEET.BLACKLIST")
                .arg(address.to_string())
                .arg("Excessive 404 Responses")
                .execute(connection.borrow_mut());
        }
    }
}

fn main() {
    let start = Instant::now();
    let log_entries = repsheet_etl::nginx::process("samples/*");
    let mut actors = repsheet_etl::processor::into_actors(log_entries);
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    apply_ruleset(actors.borrow_mut(), client.borrow());
    let duration = start.elapsed();

    println!("Processed {} actors in {:?}", actors.len(), duration);
}
