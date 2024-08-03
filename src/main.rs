use std::{env, error::Error};
use aurca::components::help;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() == 1 {
        println!("{}", help::generate()?);        
        return Ok(());
    }
    
    Ok(())
}
