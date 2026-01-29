use addparser::parse;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = parse("127.0.0.1:8080")?;
    println!("IPv4 {}:{}", addr.ip(), addr.port());

    let addr6 = parse("[2001:db8::1]:443")?;
    println!("IPv6 {}:{}", addr6.ip(), addr6.port());

    Ok(())
}
