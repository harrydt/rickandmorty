mod domain;
mod repository;

// use std::collections::HashMap;
use clap::{Parser, ValueEnum};
use core::fmt;
use std::time::Duration;

const URL: &str = "https://rickandmortyapi.com/api/";
const TIMEOUT: u64 = 3;

#[derive(Parser, Debug)]
struct Cli {
    #[clap(value_enum)]
    resource: Resource,
}

#[derive(ValueEnum, Debug, Clone)]
enum Resource {
    Character,
    Location,
    Episode,
}

impl fmt::Display for Resource {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();

    // Construct API endpoint based on the input resource
    let url = URL.to_string() + &args.resource.to_string();

    let client = reqwest::Client::new();
    let resp = client
        .get(url)
        .header("Accept", "text/plain")
        .timeout(Duration::from_secs(TIMEOUT))
        .send()
        .await?
        .text()
        .await?;

    println!("{:}", resp);
    Ok(())
}
