use method::Method;

pub struct LogEntry {
    pub address:  String,
    pub method:   Method,
    pub response: String,
}

impl LogEntry {
    pub fn valid(&self) -> bool {
        if self.address.is_empty() || self.method.is_empty() || self.response.is_empty() {
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
        let entry = LogEntry { address: "".into(), method: "".into(), response: "".into() };

        assert_eq!(entry.valid(), false);
    }

    #[test]
    fn empty_address() {
        let entry = LogEntry { address: "".into(), method: "GET".into(), response: "200".into() };

        assert_eq!(entry.valid(), false);
    }

    #[test]
    fn empty_method() {
        let entry = LogEntry { address: "1.1.1.1".into(), method: "".into(), response: "200".into() };

        assert_eq!(entry.valid(), false);
    }

    #[test]
    fn empty_response() {
        let entry = LogEntry { address: "1.1.1.1".into(), method: "GET".into(), response: "".into() };

        assert_eq!(entry.valid(), false);
    }

    #[test]
    fn valid() {
        let entry = LogEntry { address: "1.1.1.1".into(), method: "GET".into(), response: "200".into() };

        assert_eq!(entry.valid(), true);
    }
}
