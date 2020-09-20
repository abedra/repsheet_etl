pub fn valid_method(method: &String) -> bool {
    return match method.as_ref() {
        "GET" => true,
        "POST" => true,
        "PUT" => true,
        "DELETE" => true,
        "HEAD" => true,
        "OPTIONS" => true,
        "TRACE" => true,
        "CONNECT" => true,
        _ => false,
    }
}
