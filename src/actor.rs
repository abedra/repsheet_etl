use std::collections::HashMap;
use method::Method;

#[derive(Default)]
pub struct Actor {
    pub request_count: i64,
    pub invalid_request_count: i64,
    pub methods:       HashMap<Method, i64>,
    pub responses:     HashMap<String, i64>,
}
