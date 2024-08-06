use serde::Deserialize;
use isahc::prelude::*;

use crate::internal::logger;

use std::error;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct SearchResult {
    description: Option<String>,    
    maintainer: Option<String>,
    name: String,
    popularity: f32,
    
    #[serde(rename = "URL")]
    url: Option<String>,
    
    #[serde(rename = "URLPath")]
    url_path: Option<String>,
    
    version: String,
}

#[derive(Deserialize)]
struct SearchResponse {
    #[serde(rename ="resultcount")]
    result_count: u32,
    
    results: Vec<SearchResult>,
    
    #[serde(rename = "type")]
    variant: Option<String>,
    version: u32,
}

pub fn search(arguments: Vec<String>) -> Result<(), Box<dyn error::Error>> {
    if arguments.len() > 1 {
        logger::error("too many arguments, try aurca -h search for more information".to_owned(), 1)
    } else if arguments.len() == 0 {
        logger::error("no arguments provided, try aurca -h search for more information".to_owned(), 1)
    }

    let mut response = isahc::get(&format!("https://aur.archlinux.org/rpc/v5/search/{}?by=name", arguments[0]))?;
    let response_text = response.text()?;    
    
    for result in serde_json::from_str::<SearchResponse>(&response_text)?.results {
        println!("{} {}\n  {}\n", result.name, result.version, result.description.unwrap_or("No description available".to_string()));
    }
    
    Ok(())
}
