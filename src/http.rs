pub fn valid_method(method: &String) -> bool {
    match method.as_ref() {
        "GET"     => return true,
        "POST"    => return true,
        "PUT"     => return true,
        "DELETE"  => return true,
        "HEAD"    => return true,
        "OPTIONS" => return true,
        "TRACE"   => return true,
        "CONNECT" => return true,
        _         => return false,
    }
}
