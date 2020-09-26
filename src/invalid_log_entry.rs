#[derive(Eq, PartialEq, Debug)]
pub struct InvalidLogEntry {
    pub reason: String,
    pub raw: String
}