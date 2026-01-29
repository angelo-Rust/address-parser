use crate::{NetAddrError, NetAddress};
use std::net::IpAddr;

pub(crate) fn parse(input: &str) -> Result<NetAddress, NetAddrError> {
    // Handle IPv6 brackets
    let (ip_str, port_str) = if input.starts_with('[') {
        // Expect format: [IPv6]:port
        let closing_bracket = input.find(']').ok_or(NetAddrError::InvalidIp)?;
        let ip_part = &input[1..closing_bracket];
        let rest = &input[(closing_bracket + 1)..];
        let port_part = rest.strip_prefix(':').ok_or(NetAddrError::MissingPort)?;
        (ip_part, port_part)
    } else {
        // IPv4 or bare IPv6 without brackets
        input.split_once(':').ok_or(NetAddrError::MissingPort)?
    };

    // Parse IP (IPv4 or IPv6)
    let ip = ip_str
        .parse::<IpAddr>()
        .map_err(|_| NetAddrError::InvalidIp)?;

    // Parse port
    let port = port_str
        .parse::<u16>()
        .map_err(|_| NetAddrError::InvalidPort)?;
    if port == 0 {
        return Err(NetAddrError::InvalidPort);
    }

    Ok(NetAddress::new(ip, port))
}
