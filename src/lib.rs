//! address-parser - safe network address parsing

mod address;
mod error;
mod parser;

pub use crate::address::NetAddress;
pub use crate::error::NetAddrError;

/// Parse a string like "127.0.0.1:8080" or "[::1]:443"
pub fn parse(input: &str) -> Result<NetAddress, NetAddrError> {
    parser::parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_ipv4() {
        let addr = parse("127.0.0.1:8080").unwrap();
        assert_eq!(addr.ip().to_string(), "127.0.0.1");
        assert_eq!(addr.port(), 8080);
    }

    #[test]
    fn parse_ipv6() {
        let addr = parse("[2001:db8::1]:443").unwrap();
        assert_eq!(addr.ip().to_string(), "2001:db8::1");
        assert_eq!(addr.port(), 443);
    }

    #[test]
    fn missing_port() {
        let err = parse("127.0.0.1").unwrap_err();
        matches!(err, NetAddrError::MissingPort);
    }

    #[test]
    fn invalid_ip() {
        let err = parse("999.1.1.1:80").unwrap_err();
        matches!(err, NetAddrError::InvalidIp);
    }

    #[test]
    fn invalid_port() {
        let err = parse("127.0.0.1:0").unwrap_err();
        matches!(err, NetAddrError::InvalidPort);
    }
}
