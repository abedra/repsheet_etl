use address::Address;
use nginx_processing_results::NginxProcessingResults;
use actor::Actor;
use std::collections::HashMap;
use std::collections::hash_map::Entry::{Vacant, Occupied};
use hash_utils::create_or_increment;

pub fn into_actors(log_entries: NginxProcessingResults) -> HashMap<Address, Actor> {
    let mut actors: HashMap<Address, Actor> = HashMap::new();

    for log_entry in log_entries.valid {
        match actors.entry(log_entry.address) {
            Vacant(key) => {
                let mut actor = Actor { request_count: 1, ..Default::default() };
                actor.methods.insert(log_entry.method, 1);
                actor.responses.insert(log_entry.response, 1);
                key.insert(actor);
            }
            Occupied(mut actor) => {
                actor.get_mut().request_count += 1;
                create_or_increment(&mut actor.get_mut().methods, log_entry.method);
                create_or_increment(&mut actor.get_mut().responses, log_entry.response);
            }
        }
    }

    return actors;
}

#[cfg(test)]
mod tests {
    use super::*;
    use log_entry::LogEntry;
    use invalid_log_entry::InvalidLogEntry;
    use method::Method;
    use response::Response;

    #[test]
    fn emtpy_results() {
        let log_entries = NginxProcessingResults { valid: Vec::new(), invalid: Vec::new() };
        assert_eq!(into_actors(log_entries), HashMap::new());
    }

    #[test]
    fn creates_map() {
        let mut log_entries = NginxProcessingResults { valid: Vec::new(), invalid: Vec::new() };
        log_entries.valid.push(LogEntry { address: Address::from("1.1.1.1"), method: Method::from("GET"), response: Response::from("200") });
        log_entries.invalid.push(InvalidLogEntry { reason: String::from("Invalid"), raw: String::from("") });

        let actual = into_actors(log_entries);
        let mut expected: HashMap<Address, Actor> = HashMap::new();
        let mut methods = HashMap::new();
        methods.insert(Method::from("GET"), 1);
        let mut responses = HashMap::new();
        responses.insert(Response::from("200"), 1);
        expected.insert(Address::from("1.1.1.1"), Actor { request_count: 1, invalid_request_count: 0, methods, responses });
        
        assert_eq!(actual, expected);
    }
}