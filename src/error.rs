use std::fmt;

#[derive(Debug)]
pub enum NetAddrError {
    MissingPort,
    InvalidIp,
    InvalidPort,
}

impl fmt::Display for NetAddrError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NetAddrError::MissingPort => write!(f, "missing port"),
            NetAddrError::InvalidIp => write!(f, "invalid IP address"),
            NetAddrError::InvalidPort => write!(f, "invalid port number (0-65535)"),
        }
    }
}

impl std::error::Error for NetAddrError {}
