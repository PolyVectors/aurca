use std::process;

const RED: &'static str = "\u{001B}[31m";
const YELLOW: &'static str = "\u{001B}[33m";

const BOLD: &'static str = "\u{001B}[1m";
const RESET: &'static str = "\u{001B}[0m";

pub fn error<T: AsRef<str>>(message: T, exit_code: i32) {
    println!("{RED}{BOLD}error:{RESET}{RESET} {}", message.as_ref());
    process::exit(exit_code);
}

pub fn warn<T: AsRef<str>>(message: T) {
    println!("{YELLOW}{BOLD}warning:{RESET}{RESET} {}", message.as_ref());
}
