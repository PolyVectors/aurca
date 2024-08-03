use std::error;
use crate::lib::component::{Component, ComponentType};

pub const OPTIONS: &[&Component] = &[
    &Component::new("dependencies", "D", "Apply command with dependencies", ComponentType::Option),
    &Component::new("help", "h", "WHAT? HELP ME!", ComponentType::Option),
    &Component::new("quiet", "Q", "Show less information", ComponentType::Option),
    &Component::new("verbose", "V", "Show more information", ComponentType::Option)
];

pub const COMMANDS: &[&Component] = &[
    &Component::new("query", "q", "Query the package database", ComponentType::Command),
    &Component::new("remove", "r", "Remove the package(s) from the system", ComponentType::Command),
    &Component::new("sync", "s", "Synchronise package(s)", ComponentType::Command)
];

pub fn generate_component_list(array: &[&Component]) -> Result<Vec<String>, Box<dyn error::Error>> {
    let formatted_options: Vec<(usize, String)> = array
        .iter()
        .enumerate()
        .map(|(i, o)| (i, if o.variant == ComponentType::Option { format!("-{}, --{}", o.alias, o.name) } else {  format!("{}, {}", o.name, o.alias) }))
        .collect();

    let longest = formatted_options
        .iter()
        .map(|(_i, f)| f.len())
        .max()
        .expect("No items in vector");

    Ok(formatted_options
        .iter()
        .map(|(i, f)| format!(
            "  {f}{}  {}",
            " ".repeat(longest - f.len()), array[*i].description
        ))
        .collect::<Vec<String>>()
    )
}

pub fn generate() -> Result<String, Box<dyn error::Error>> {
    Ok(format!(
        "Simple AUR helper\n\nUsage: aurca [options] [command]\n\nOptions:\n{}\n\nCommands:\n{} ",
        generate_component_list(OPTIONS)?.join("\n"), generate_component_list(COMMANDS)?.join("\n")
    ))
}
