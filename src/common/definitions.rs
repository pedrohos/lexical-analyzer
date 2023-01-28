use std::fmt::{Debug, format};
use std::string::ToString;
use std::thread::sleep;
use regex::Regex;
use lazy_static::lazy_static;

#[allow(unused_assignments)]

#[derive(Copy, Clone, Debug)]
pub enum Constant {
    INTEGER,
    IDENTIFIER,
    IF,
    ELSE,
    OPEN_BRACE,
    CLOSE_BRACE,
    OPEN_PAREN,
    CLOSE_PAREN,
    SEMICOLON,
    RETURN,
    INT
}

lazy_static! {
    pub static ref PATTERNS: Vec<(Regex, Constant)> = vec![
        (Regex::new(r"^[0-9]$").unwrap(), Constant::INTEGER),
        (Regex::new("^int$").unwrap(), Constant::INT),
        (Regex::new(r"^return$").unwrap(), Constant::RETURN),
        (Regex::new(r"^if$").unwrap(), Constant::IF),
        (Regex::new(r"^else$").unwrap(), Constant::ELSE),
        (Regex::new(r"^\{$").unwrap(), Constant::OPEN_BRACE),
        (Regex::new(r"^\}$").unwrap(), Constant::CLOSE_BRACE),
        (Regex::new(r"^\($").unwrap(), Constant::OPEN_PAREN),
        (Regex::new(r"^\)$").unwrap(), Constant::CLOSE_PAREN),
        (Regex::new(r"^;$").unwrap(), Constant::SEMICOLON),
        (Regex::new("^[a-zA-Z]*$").unwrap(), Constant::IDENTIFIER)
    ];
}
