use std::fs::File;
use std::error::Error;
use std::net::Ipv6Addr;

fn run() -> Result<(), Box<dyn Error>> { // A trait object, Box, represents any type that implements Error.
    let _f = File::open("invisible.txt")?;

    let _localhost = "::1"
        .parse::<Ipv6Addr>()?;

    Ok(())
}
