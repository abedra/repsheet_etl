use log_entry::LogEntry;
use invalid_log_entry::InvalidLogEntry;

pub struct NginxProcessingResults {
    pub valid: Vec<LogEntry>,
    pub invalid: Vec<InvalidLogEntry>,
}
