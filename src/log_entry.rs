use method::Method;
use response::Response;
use address::Address;

pub struct LogEntry {
    pub address:  Address,
    pub method:   Method,
    pub response: Response,
}

impl LogEntry {
    pub fn is_valid(&self) -> bool {
        self.address.is_valid()
            && self.method.is_valid()
            && self.response.is_valid()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_empty() {
        let entry = LogEntry {
            address: Address::from(""),
            method: Method::from(""),
            response: Response::from("")
        };

        assert_eq!(entry.is_valid(), false);
    }

    #[test]
    fn empty_address() {
        let entry = LogEntry {
            address: Address::from(""),
            method: Method::from("GET"),
            response: Response::from("200")
        };

        assert_eq!(entry.is_valid(), false);
    }

    #[test]
    fn empty_method() {
        let entry = LogEntry {
            address: Address::from("1.1.1.1"),
            method: Method::from(""),
            response: Response::from("200")
        };

        assert_eq!(entry.is_valid(), false);
    }

    #[test]
    fn empty_response() {
        let entry = LogEntry {
            address: Address::from("1.1.1.1"),
            method: Method::from("GET"),
            response: Response::from("")
        };

        assert_eq!(entry.is_valid(), false);
    }

    #[test]
    fn valid() {
        let entry = LogEntry {
            address: Address::from("1.1.1.1"),
            method: Method::from("GET"),
            response: Response::from("200")
        };

        assert_eq!(entry.is_valid(), true);
    }
}
