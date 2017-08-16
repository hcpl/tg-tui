use std::iter::FromIterator;

use pom::TextInput;
use pom::parser::*;

use commands::Command;
use error;


fn parse_command<C: Command>(input: &str) -> error::Result<C> {
    let parser = sym(':') + none_of(" ").repeat(1..);
    let mut input = TextInput::new(input);
    let res = parser.parse(&mut input)?;

    assert_eq!(res.0, ':');
    let command_name = String::from_iter(res.1);

    unimplemented!()
}
