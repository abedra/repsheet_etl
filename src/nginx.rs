use std::collections::HashMap;
use std::collections::hash_map::Entry::{Vacant, Occupied};

use actor::Actor;
use log_entry::LogEntry;
use http;
use hash_utils::create_or_increment;

pub fn process(actors: &mut HashMap<String, Actor>, line: &str) {
    let parts: Vec<&str> = line.split(' ').collect();
    let log_entry = LogEntry { address: parts[0].to_string(), method: parts[5].replace("\"", "").to_string(), response: parts[8].to_string() };
    if !http::valid_method(&log_entry.method) {
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