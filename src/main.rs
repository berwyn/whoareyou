fn main() -> std::io::Result<()> {
    let host = hostname::get()?;
    let host = match host.to_str() {
        Some(value) => value,
        None => std::process::exit(1),
    };

    println!("{}", host);

    Ok(())
}
