use either::Either;
use either::Either::{Left, Right};

use log_entry::LogEntry;
use invalid_log_entry::InvalidLogEntry;

use method::Method;
use response::Response;
use address::Address;
use nginx_processing_results::NginxProcessingResults;
use file::{collect_log_files, read_log_files};

// pub fn process(actors: &mut HashMap<Address, Actor>, line: &str) {
//     let parts: Vec<&str> = line.split(' ').collect();
//     let log_entry = LogEntry {
//         address: Address::from(parts[0]),
//         method: Method::from(parts[5]),
//         response: Response::from(parts[8]),
//     };
//
//     if !log_entry.method.is_valid() {
//         match actors.entry(log_entry.address) {
//             Vacant(key) => { key.insert(Actor { invalid_request_count: 1, ..Default::default() }); }
//             Occupied(mut actor) => actor.get_mut().invalid_request_count += 1,
//         }
//         return;
//     }
//
//     match actors.entry(log_entry.address) {
//         Vacant(key) => {
//             let mut actor = Actor { request_count: 1, ..Default::default() };
//             actor.methods.insert(log_entry.method, 1);
//             actor.responses.insert(log_entry.response, 1);
//             key.insert(actor);
//         }
//         Occupied(mut actor) => {
//             actor.get_mut().request_count += 1;
//             create_or_increment(&mut actor.get_mut().methods, log_entry.method);
//             create_or_increment(&mut actor.get_mut().responses, log_entry.response);
//         }
//     }
// }

#[allow(dead_code)]
pub fn process(pattern: &str) -> NginxProcessingResults {
    let log_entries = read_log_files(collect_log_files(pattern));
    let mut valid_entries: Vec<LogEntry> = Vec::new();
    let mut invalid_entries: Vec<InvalidLogEntry> = Vec::new();

    for entry in log_entries {
        match process_line(entry.as_str()) {
            Either::Left(result) => invalid_entries.push(result),
            Either::Right(result) => valid_entries.push(result)
        }
    }

    NginxProcessingResults{valid: valid_entries, invalid: invalid_entries}
}

pub fn process_line(line: &str) -> Either<InvalidLogEntry, LogEntry> {
    let parts: Vec<&str> = line.split(' ').collect();

    if !(parts.len() > 9 as usize) {
        return Left(InvalidLogEntry { reason: String::from("Not enough parts"), raw: line.to_string() });
    }

    let log_entry = LogEntry {
        address: Address::from(parts[0]),
        method: Method::from(parts[5]),
        response: Response::from(parts[8]),
    };

    return if log_entry.is_valid() {
        Right(log_entry)
    } else {
        Left(InvalidLogEntry { reason: String::from("Invalid Log Entry"), raw: line.to_string() })
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_line() {
        let line = "";

        assert_eq!(process_line(line).unwrap_left(), InvalidLogEntry { reason: String::from("Not enough parts"), raw: String::from("") });
    }

    #[test]
    fn invalid_address() {
        let line = "1.1 - - [10/Nov/2017:03:31:38 -0500] \"GET / HTTP/1.1\" 200 2947 \"-\" \"Mozilla/5.0 (compatible; Baiduspider/2.0; +http://www.baidu.com/search/spider.html)\"";

        assert_eq!(process_line(line).unwrap_left(), InvalidLogEntry { reason: String::from("Invalid Log Entry"), raw: String::from(line) });
    }

    #[test]
    fn invalid_method() {
        let line = "220.181.108.119 - - [10/Nov/2017:03:31:38 -0500] \"BAD / HTTP/1.1\" 200 2947 \"-\" \"Mozilla/5.0 (compatible; Baiduspider/2.0; +http://www.baidu.com/search/spider.html)\"";

        assert_eq!(process_line(line).unwrap_left(), InvalidLogEntry { reason: String::from("Invalid Log Entry"), raw: String::from(line) });
    }

    #[test]
    fn invalid_response_code() {
        let line = "220.181.108.119 - - [10/Nov/2017:03:31:38 -0500] \"GET / HTTP/1.1\" 9999 2947 \"-\" \"Mozilla/5.0 (compatible; Baiduspider/2.0; +http://www.baidu.com/search/spider.html)\"";

        assert_eq!(process_line(line).unwrap_left(), InvalidLogEntry { reason: String::from("Invalid Log Entry"), raw: String::from(line) });
    }

    #[test]
    fn valid_line() {
        let line = "220.181.108.119 - - [10/Nov/2017:03:31:38 -0500] \"GET / HTTP/1.1\" 200 2947 \"-\" \"Mozilla/5.0 (compatible; Baiduspider/2.0; +http://www.baidu.com/search/spider.html)\"";

        assert_eq!(process_line(line).unwrap_right().address, Address::from("220.181.108.119"));
    }
}

