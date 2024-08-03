use std::{env, error::Error};

enum ComponentType {
    Option,
    Command,
}

struct Component<'a> {
    name: &'a str,
    alias: &'a str,
    description: &'a str,
    variant: ComponentType,
}

impl <'a> Component <'a> {
    pub const fn new(
        name: &'a str,
        alias: &'a str,
        description: &'a str,
        variant: ComponentType
    ) -> Self {
        Self { name, alias, description, variant }
    }
}

const OPTIONS: &[&Component] = &[
    &Component::new("dependencies", "D", "Apply command with dependencies", ComponentType::Option),
    &Component::new("help", "h", "Show this screen", ComponentType::Option),
    &Component::new("quiet", "Q", "Show less information", ComponentType::Option),
    &Component::new("verbose", "V", "Show more information", ComponentType::Option)
];

const COMMANDS: &[&Component] = &[
    &Component::new("query", "q", "Query the package database", ComponentType::Command),
    &Component::new("remove", "r", "Remove the package(s) from the system", ComponentType::Command),
    &Component::new("sync", "s", "Synchronise package(s)", ComponentType::Command)
];

fn parse_args(args: &Vec<String>) -> Vec<&str> {
    todo!();
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() == 1 {
      return Ok(());
    }
    
    Ok(())
}
