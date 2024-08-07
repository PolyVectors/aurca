use crate::core::definitions::{
    cli::{COMMANDS, OPTIONS},
    parameter::{Parameter, ParameterType},
};
use std::error;

const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");

fn generate_parameter_list(array: &[&Parameter]) -> Result<Vec<String>, Box<dyn error::Error>> {
    let formatted_parameters: Vec<(usize, String)> = array
        .iter()
        .enumerate()
        .map(|(i, p)| {
            (
                i,
                if p.variant == ParameterType::Option {
                    format!("-{}, --{}", p.alias, p.name)
                } else {
                    format!("{}, {}", p.name, p.alias)
                },
            )
        })
        .collect();

    let longest = formatted_parameters
        .iter()
        .map(|(_i, f)| f.len())
        .max()
        .expect("No items in vector");

    Ok(formatted_parameters
        .iter()
        .map(|(i, f)| {
            format!(
                "  {f}{}  {}",
                " ".repeat(longest - f.len()),
                array[*i].description
            )
        })
        .collect::<Vec<String>>())
}

pub fn generate(command: Option<String>) -> Result<String, Box<dyn error::Error>> {
    if command.is_some() {
        todo!();
    }

    Ok(format!(
        "{}\n\nUsage: aurca [options] [command?] [...]\n\nOptions:\n{}\n\nCommands:\n{} ",
        DESCRIPTION,
        generate_parameter_list(OPTIONS)?.join("\n"),
        generate_parameter_list(COMMANDS)?.join("\n")
    ))
}
