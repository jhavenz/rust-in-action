use std::{error, fmt};
use std::fs::File;
use std::io;
use std::net;
use std::net::Ipv6Addr;

#[derive(Debug)]
enum UpstreamError{
    IO(io::Error),
    Parsing(net::AddrParseError),
}

impl fmt::Display for UpstreamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::Error for UpstreamError { }

fn run() -> Result<(), UpstreamError> {
    let _f = File::open("invisible.txt")
        .map_err(UpstreamError::IO)?;

    let _localhost = "::1"
        .parse::<Ipv6Addr>()
        .map_err(UpstreamError::Parsing)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let _ = run().unwrap();
        //assert!(run().is_err());
    }
}
