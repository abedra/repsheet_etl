#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Method(String);

impl From<&str> for Method {
    fn from(value: &str) -> Self {
        return Method(value.replace("\"", ""));
    }
}

impl Method {
    pub fn is_empty(&self) -> bool {
        return self.0.is_empty();
    }

    pub fn is_valid(&self) -> bool {
        return match self.0.as_str() {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn removes_quote() {
        assert_eq!(Method::from("\"GET"), Method::from("GET"));
    }
}
