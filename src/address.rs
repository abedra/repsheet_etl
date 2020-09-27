use std::net::{IpAddr, Ipv4Addr};
use std::borrow::Borrow;
use std::fmt::{Debug, Formatter, Display};
use core::fmt;
use std::slice::Iter;
use itertools::Itertools;

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct Address(IpAddr);

impl Display for Address {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        return match self.0 {
            IpAddr::V4(v4) => write!(f, "{}", join(v4.octets().iter(), ".")),
            IpAddr::V6(v6) => write!(f, "{}", join(v6.octets().iter(), ":"))
        }
    }
}

impl From<&str> for Address {
    fn from(value: &str) -> Self {
        let address = value.parse::<IpAddr>();
        return match address {
            Ok(result) => Address(result),
            Err(_) => Address(IpAddr::from(Ipv4Addr::new(0, 0, 0, 0)))
        }
    }
}

impl Address {
    pub fn is_valid(&self) -> bool {
        !self.0.eq(IpAddr::from(Ipv4Addr::new(0, 0, 0, 0)).borrow())
    }
}

fn join(a: Iter<u8>, sep: &str) -> String {
    a.map(|&i| i.to_string()).join(sep)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_v4() {
        let actual = Address::from("1.1.1.1");
        assert_eq!(actual.is_valid(), true);
    }

    #[test]
    fn valid_v6() {
        let actual = Address::from("::1");
        assert_eq!(actual.is_valid(), true);
    }

    #[test]
    fn invalid() {
        let actual = Address::from("1.1");
        assert_eq!(actual.is_valid(), false);
    }
}
