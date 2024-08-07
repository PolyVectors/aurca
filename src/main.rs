use aurca::core::parser;
use std::error;

fn main() -> Result<(), Box<dyn error::Error>> {
    parser::parse_args()?;
    Ok(())
}
