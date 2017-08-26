use std::iter::FromIterator;

use pom::{Parser, TextInput};
use pom::parser::*;

use commands::Command;
use error;


fn space() -> Parser<char, ()> {
    one_of(" \t\r\n").repeat(0..).discard()
}

fn command_name() -> Parser<char, String> {
    is_a(char::is_alphabetic).repeat(1..).map(String::from_iter)
}

fn arg() -> Parser<char, String> {
    none_of(" ").repeat(1..).map(String::from_iter)
}

fn command() -> Parser<char, (String, Vec<String>)> {
    sym(':') * space() * command_name() + (space() * arg()).repeat(0..)
}


pub fn parse_command<C: Command>(input: &str) -> error::Result<C> {
    let mut input = TextInput::new(input);
    let (command, args) = command().parse(&mut input)?;

    C::try_from_str(&command, args.iter().map(String::as_str))
}
