use std::borrow::Borrow;

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct Response(u16);

impl From<&str> for Response {
    fn from(value: &str) -> Self {
        let result = value.parse::<u16>();
        return match result {
            Ok(result) => Response(result),
            Err(_) => Response(0)
        }
    }
}

impl Response {
    pub fn is_valid(&self) -> bool {
        let range = 100..=599;
        return range.contains(self.0.borrow());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn below_valid_range() {
        let actual = Response::from("0");
        assert_eq!(actual.is_valid(), false);
    }

    #[test]
    fn at_range_beginning() {
        let actual = Response::from("100");
        assert_eq!(actual.is_valid(), true);
    }

    #[test]
    fn in_range() {
        let actual = Response::from("200");
        assert_eq!(actual.is_valid(), true);
    }

    #[test]
    fn at_range_end() {
        let actual = Response::from("599");
        assert_eq!(actual.is_valid(), true);
    }

    #[test]
    fn above_range() {
        let actual = Response::from("600");
        assert_eq!(actual.is_valid(), false);
    }
}