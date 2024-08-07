pub struct Parameter<'a> {
    pub name: &'a str,
    pub alias: &'a str,
    pub description: &'a str,
    pub variant: ParameterType,
}

impl<'a> Parameter<'a> {
    pub const fn new(
        name: &'a str,
        alias: &'a str,
        description: &'a str,
        variant: ParameterType,
    ) -> Self {
        Self {
            name,
            alias,
            description,
            variant,
        }
    }
}

#[derive(PartialEq)]
pub enum ParameterType {
    Option,
    Command,
}

pub const OPTIONS: &[&Parameter] = &[
    &Parameter::new(
        "dependencies",
        "D",
        "Apply command with dependencies",
        ParameterType::Option,
    ),
    &Parameter::new(
        "help",
        "h",
        "WHAT? HELP ME! (Shows this screen)",
        ParameterType::Option,
    ),
    &Parameter::new(
        "quiet",
        "Q",
        "Show less install information",
        ParameterType::Option,
    ),
    &Parameter::new(
        "verbose",
        "v",
        "Show more install information",
        ParameterType::Option,
    ),
    &Parameter::new(
        "version",
        "V",
        "Display the version number",
        ParameterType::Option,
    ),
];

pub const COMMANDS: &[&Parameter] = &[
    &Parameter::new(
        "search",
        "s",
        "Search the package database",
        ParameterType::Command,
    ),
    &Parameter::new(
        "remove",
        "r",
        "Remove the package(s) from the system",
        ParameterType::Command,
    ),
    &Parameter::new(
        "sync",
        "s",
        "Synchronise package(s)",
        ParameterType::Command,
    ),
];
