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
    operation: Operation,
    ids: Option<Vec<i32>>,
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

#[derive(ValueEnum, Debug, Clone)]
enum Operation {
    GetAll,
    Get,
    GetMultiple,
}

#[derive(Debug)]
struct CustomError(String);

#[tokio::main]
async fn main() -> Result<(), CustomError> {
    let args = Cli::parse();

    let url = construct_url(args)?;

    let client = reqwest::Client::new();
    let resp = client
        .get(url)
        .header("Accept", "text/plain")
        .timeout(Duration::from_secs(TIMEOUT))
        .send()
        .await
        .map_err(|err| CustomError(format!("Error with request call {}", err)))?
        .text()
        .await
        .map_err(|err| CustomError(format!("Error with getting response text {}", err)))?;

    println!("{:}", resp);
    Ok(())
}

// Construct API endpoint based on the input resource
fn construct_url(args: Cli) -> Result<String, CustomError> {
    let url = URL.to_string() + &args.resource.to_string();
    match args.operation {
        Operation::GetAll => Ok(url),
        Operation::Get => match args.ids {
            Some(ids) => Ok(url + "/" + &ids[0].to_string()),
            None => Err(CustomError(String::from("missing id"))),
        },
        Operation::GetMultiple => match args.ids {
            Some(ids) => Ok(url + "/" + &format!("{:?}", ids)),
            None => Err(CustomError(String::from("missing ids (for example 1 2 3"))),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_url_get_all_character() -> Result<(), CustomError> {
        let args = Cli {
            resource: Resource::Character,
            operation: Operation::GetAll,
            ids: None,
        };
        let url = construct_url(args)?;
        assert_eq!(
            String::from("https://rickandmortyapi.com/api/character"),
            url
        );
        Ok(())
    }

    #[test]
    fn test_construct_url_get_character() -> Result<(), CustomError> {
        let ids = vec![5];
        let args = Cli {
            resource: Resource::Character,
            operation: Operation::Get,
            ids: Some(ids),
        };
        let url = construct_url(args)?;
        assert_eq!(
            String::from("https://rickandmortyapi.com/api/character/5"),
            url
        );
        Ok(())
    }

    #[test]
    fn test_construct_url_get_multiple_character() -> Result<(), CustomError> {
        let ids = vec![2, 3, 5];
        let args = Cli {
            resource: Resource::Character,
            operation: Operation::GetMultiple,
            ids: Some(ids),
        };
        let url = construct_url(args)?;
        assert_eq!(
            String::from("https://rickandmortyapi.com/api/character/[2, 3, 5]"),
            url
        );
        Ok(())
    }
}
