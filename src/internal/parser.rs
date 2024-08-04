use crate::{parameters::help, internal::parameter::{Parameter, ParameterType, OPTIONS, COMMANDS}};
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
        // todo: make this cleaner (match with tuple?)
        if !options.contains(&arg) && !commands.contains(&arg) {
            // make dedicated logging function
            println!("Unknown argument '{}', try aurca -h for help.", &arg);
            process::exit(1);
        } else if specified_options.contains(&arg) && !commands.contains(&arg) {
            println!("Options cannot be specified more than once, try aurca -h for help.");
            process::exit(1);
        } else if specified_command.is_some() && commands.contains(&arg) {
            println!("Commands cannot be specified more than once, try aurca -h for help.");
            process::exit(1);
        } else if options.contains(&arg) && specified_command.is_some() {
            println!("Cannot specify option after command, try aurca -h for help.");
            process::exit(1);
        } else if options.contains(&arg) && !commands.contains(&arg) {
            specified_options.push(arg.clone());
        } else if commands.contains(&arg) && !options.contains(&arg) {
            specified_command = Some(arg.clone());
        }
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
