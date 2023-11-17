#![allow(dead_code)]

use colored::{Color, Colorize};

/// Prints the given message to the console and aborts the process.
pub fn abort(msg: &str) -> ! {
    error(msg);
    std::process::exit(1);
}

pub fn info(msg: &str, color: Color) {
    println!("{}: {}", "Info".bold().color(color), msg);
}

pub fn message(msg: &str, color: Color) {
    println!("Message: {}", msg.bold().color(color));
}

pub fn error(msg: &str) {
    println!("{}: {}", "Error".bold().color(Color::Red), msg);
}

pub fn success(msg: &str) {
    println!("{}: {}", "Success".bold().color(Color::Green), msg);
}

pub fn warn(msg: &str) {
    println!("{}: {}", "Warning".bold().color(Color::Yellow), msg);
}
