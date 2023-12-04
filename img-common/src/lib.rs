use std::error::Error;

use better_term::{Color, flush_styles};

pub fn info<S: Into<String>>(msg: S) {
    println!("{}{}", Color::White, msg.into());
    flush_styles();
}

pub fn error<S: Into<String>>(msg: S) {
    println!("{}{}", Color::Red, msg.into());
    flush_styles();
}

pub fn error_with_err<S: Into<String>, E: Error>(msg: S, err: E) {
    println!("{}{}: {}{}", Color::Red, msg.into(), Color::BrightWhite, err);
    flush_styles();
}

pub fn highlighted_info<S: Into<String>, H: Into<String>>(msg: S, highlighted: H) {
    println!("{}{} {}{}", Color::White, msg.into(), Color::BrightCyan, highlighted.into());
    flush_styles();
}

pub fn conn_log<IP: Into<String>, S: Into<String>>(ip: IP, msg: S) {
    println!("{}[{}{}{}] {}{}", Color::BrightBlack, Color::BrightCyan, ip.into(), Color::BrightBlack, Color::White, msg.into());
    flush_styles();
}