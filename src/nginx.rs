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

pub fn process_line(line: &str) -> Result<LogEntry, String> {
    let parts: Vec<&str> = line.split(' ').collect();

    if !(parts.len() > 9 as usize) {
        return Err(String::from("Not enough parts"))
    }

    let log_entry = LogEntry {
        address: Address::from(parts[0]),
        method: Method::from(parts[5]),
        response: Response::from(parts[8])
    };

    return if log_entry.valid() {
        Ok(log_entry)
    } else {
        Err(String::from("Invalid Log Entry"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_line() {
        let line = "";
        let result = process_line(line);

        match result {
            Ok(_) => assert!(false),
            Err(s) => assert_eq!(s, "Not enough parts")
        }
    }

    #[test]
    fn invalid_address() {
        let line = "1.1 - - [10/Nov/2017:03:31:38 -0500] \"GET / HTTP/1.1\" 200 2947 \"-\" \"Mozilla/5.0 (compatible; Baiduspider/2.0; +http://www.baidu.com/search/spider.html)\"";
        let result = process_line(line);

        match result {
            Ok(_) => assert!(false),
            Err(s) => assert_eq!(s, "Invalid Log Entry")
        }
    }

    #[test]
    fn invalid_method() {
        let line = "220.181.108.119 - - [10/Nov/2017:03:31:38 -0500] \"BAD / HTTP/1.1\" 200 2947 \"-\" \"Mozilla/5.0 (compatible; Baiduspider/2.0; +http://www.baidu.com/search/spider.html)\"";
        let result = process_line(line);

        match result {
            Ok(_) => assert!(false),
            Err(s) => assert_eq!(s, "Invalid Log Entry")
        }
    }

    #[test]
    fn invalid_response_code() {
        let line = "220.181.108.119 - - [10/Nov/2017:03:31:38 -0500] \"GET / HTTP/1.1\" 9999 2947 \"-\" \"Mozilla/5.0 (compatible; Baiduspider/2.0; +http://www.baidu.com/search/spider.html)\"";
        let result = process_line(line);

        match result {
            Ok(_) => assert!(false),
            Err(s) => assert_eq!(s, "Invalid Log Entry")
        }
    }

    #[test]
    fn valid_line() {
        let line = "220.181.108.119 - - [10/Nov/2017:03:31:38 -0500] \"GET / HTTP/1.1\" 200 2947 \"-\" \"Mozilla/5.0 (compatible; Baiduspider/2.0; +http://www.baidu.com/search/spider.html)\"";
        let result = process_line(line);

        match result {
            Ok(entry) => assert_eq!(entry.address, Address::from("220.181.108.119")),
            Err(_) => assert!(false)
        }
    }
}

