# netaddr

![Rust](https://img.shields.io/badge/language-Rust-orange)
![License](https://img.shields.io/badge/license-MIT-green)

A small Rust library demonstrating **clean public API design**, strong typing,
and safe parsing of network addresses.

## ✨ Features

- Minimal and explicit public API
- Strong typing using `Ipv4Addr` and `u16`
- No panics, explicit error handling
- Internal parsing logic hidden from users

## 📦 Example

```rust
use netaddr::NetAddress;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: NetAddress = "192.168.1.10:8080".parse()?;
    println!("{}:{}", addr.ip(), addr.port());
    Ok(())
}
