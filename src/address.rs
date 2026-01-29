use std::net::IpAddr;

#[derive(Debug, PartialEq, Eq)]
pub struct NetAddress {
    ip: IpAddr, // supports IPv4 and IPv6
    port: u16,
}

impl NetAddress {
    // Getters (public API)
    pub fn ip(&self) -> IpAddr {
        self.ip
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    // Constructor for internal use
    pub(crate) fn new(ip: IpAddr, port: u16) -> Self {
        Self { ip, port }
    }
}
