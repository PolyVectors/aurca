use std::process;

const RED: &'static str = "\u{001B}[31m";
const BOLD: &'static str = "\u{001B}[1m";
const RESET: &'static str = "\u{001B}[0m";

pub fn error(message: String, exit_code: i32) {
    println!("{}{}error{}{}: {}", RED, BOLD, RESET, RESET, message);
    process::exit(exit_code);
}
