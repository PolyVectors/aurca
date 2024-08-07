use crate::internal::{log, search_response::SearchResponse};
use isahc::prelude::*;

use std::{
    error,
    process::{Command, Stdio},
};

pub fn search(arguments: Vec<String>) -> Result<(), Box<dyn error::Error>> {
    if arguments.len() > 1 {
        log::warn("too many arguments, other arguments will not be searched")
    } else if arguments.len() == 0 {
        log::error("no arguments provided, try aurca -h search", 1)
    }

    let mut response = isahc::get(&format!(
        "https://aur.archlinux.org/rpc/v5/search/{}?by=name",
        arguments[0]
    ))?;
    let response_text = response.text()?;

    let deserialized_response = serde_json::from_str::<SearchResponse>(&response_text)?;
    let deserialized_results = deserialized_response.results.into_iter().enumerate();

    let result_count = deserialized_response.result_count;

    for (index, result) in deserialized_results {
        let installed = match Command::new("pacman")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .arg("-Q")
            .arg(&result.name)
            .status()?
            .code()
            .unwrap_or(1)
        {
            0 => true,
            _ => false,
        };

        println!(
            "{} {} {}\n  {}{}",
            result.name,
            result.version,
            if installed { "[Installed]" } else { "" },
            result
                .description
                .unwrap_or("No description available".to_string()),
            if index + 1 == result_count { "" } else { "\n" },
        );
    }

    Ok(())
}
