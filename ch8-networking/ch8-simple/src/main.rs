use std::error::Error;

use reqwest;

fn main() -> Result<(), Box<dyn Error>>{
    let url = "https://github.com";
    let mut response = reqwest::get(url)?;

    let content = response.text()?;
    println!("Content: {}", content);

    Ok(())
}
