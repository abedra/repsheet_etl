use std::collections::HashMap;
use method::Method;
use response::Response;

#[derive(Default)]
pub struct Actor {
    pub request_count: i64,
    pub invalid_request_count: i64,
    pub methods:       HashMap<Method, i64>,
    pub responses:     HashMap<Response, i64>,
}
