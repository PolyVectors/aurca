use crate::{parameters::help, internal::{logger, parameter::{Parameter, ParameterType, OPTIONS, COMMANDS}}};
use std::{env, error, process};

fn get_parameters<'a>(array: &[&Parameter]) -> Result<Vec<String>, Box<dyn error::Error>> {    
    let components = array
        .to_vec()
        .iter()
        .map(|c| {
            if c.variant == ParameterType::Option {
                format!("--{} -{}", c.name, c.alias)
            } else {
                format!("{} {}", c.name, c.alias)
            }
        })
        .collect::<Vec<String>>()
        .join(" ");

    let components: Vec<String> = components
        .split(" ")
        .collect::<Vec<&str>>()
        .iter()
        .map(|c| c.to_string())
        .collect();
       
    Ok(components)
}

pub fn parse_args() -> Result<(), Box<dyn error::Error>> {   
    let args: Vec<String> = env::args().collect();
    
    let options = get_parameters(OPTIONS)?;
    let commands = get_parameters(COMMANDS)?;

    let mut specified_options = Vec::new();
    let mut specified_command: Option<String> = None;

    for arg in args.into_iter().skip(1) {
        match (
            options.contains(&arg), commands.contains(&arg),
            specified_options.contains(&arg), specified_command.is_some()
        ) {
            (false, false, _, _) => logger::error(format!("unexpected argument '{}'", &arg), 1),
            (_, false, true, _) => logger::error(format!("duplicate option '{}'", &arg), 1),
            (_, true, _, true) => logger::error(format!("duplicate command '{}'", &arg), 1),
            (true, _, _, true) => logger::error("option specified after command".to_owned(), 1),
            (true, _, _, _) => specified_options.push(arg.clone()),
            (_, true, _, _) => specified_command = Some(arg.clone()),
        };
    }

    if specified_options.len() == 0 && specified_command.is_none() {
        println!("{}", help::generate(specified_command)?);
        process::exit(0);
    }

    let specified_options: Vec<&str> = specified_options
        .iter()
        .map(|s: &String| s.as_str())
        .collect();

    for option in specified_options {
        match option {
            "-h" | "--help" => {
                println!("{}", help::generate(specified_command)?);
                process::exit(0);
            },
            &_ => {},
        }
    }

    Ok(())
}
