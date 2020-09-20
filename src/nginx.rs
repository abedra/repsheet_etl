use std::collections::HashMap;
use std::collections::hash_map::Entry::{Vacant, Occupied};

use actor::Actor;
use log_entry::LogEntry;
use hash_utils::create_or_increment;
use method::Method;
use response::Response;
use address::Address;

pub fn process(actors: &mut HashMap<Address, Actor>, line: &str) {
    let parts: Vec<&str> = line.split(' ').collect();
    let log_entry = LogEntry {
        address: Address::from(parts[0]),
        method: Method::from(parts[5]),
        response: Response::from(parts[8])
    };

    if !log_entry.method.is_valid() {
        match actors.entry(log_entry.address) {
            Vacant(key) => { key.insert(Actor { invalid_request_count: 1, ..Default::default() }); },
            Occupied(mut actor) => actor.get_mut().invalid_request_count += 1,
        }
        return;
    }

    if log_entry.valid() {
        match actors.entry(log_entry.address) {
            Vacant(key) => {
                let mut actor = Actor { request_count: 1, ..Default::default() };
                actor.methods.insert(log_entry.method, 1);
                actor.responses.insert(log_entry.response, 1);
                key.insert(actor);
            },
            Occupied(mut actor) => {
                actor.get_mut().request_count += 1;
                create_or_increment(&mut actor.get_mut().methods, log_entry.method);
                create_or_increment(&mut actor.get_mut().responses, log_entry.response);
            },
        }
    } else {
        //TODO: Do something useful here.
    }
}
