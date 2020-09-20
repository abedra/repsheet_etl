use method::Method;
use response::Response;

pub struct LogEntry {
    pub address:  String,
    pub method:   Method,
    pub response: Response,
}

impl LogEntry {
    pub fn valid(&self) -> bool {
        if self.address.is_empty() || !self.method.is_valid() || !self.response.is_valid() {
            return false;
        }

        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_empty() {
        let entry = LogEntry {
            address: String::from(""),
            method: Method::from(""),
            response: Response::from("")
        };

        assert_eq!(entry.valid(), false);
    }

    #[test]
    fn empty_address() {
        let entry = LogEntry {
            address: String::from(""),
            method: Method::from("GET"),
            response: Response::from("200")
        };

        assert_eq!(entry.valid(), false);
    }

    #[test]
    fn empty_method() {
        let entry = LogEntry {
            address: String::from("1.1.1.1"),
            method: Method::from(""),
            response: Response::from("200")
        };

        assert_eq!(entry.valid(), false);
    }

    #[test]
    fn empty_response() {
        let entry = LogEntry {
            address: String::from("1.1.1.1"),
            method: Method::from("GET"),
            response: Response::from("")
        };

        assert_eq!(entry.valid(), false);
    }

    #[test]
    fn valid() {
        let entry = LogEntry {
            address: String::from("1.1.1.1"),
            method: Method::from("GET"),
            response: Response::from("200")
        };

        assert_eq!(entry.valid(), true);
    }
}
