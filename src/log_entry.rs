pub struct LogEntry {
    pub address:  String,
    pub method:   String,
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
