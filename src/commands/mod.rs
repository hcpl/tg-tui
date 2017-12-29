pub mod parser;

pub use self::parser::parse_command;


use cursive::Cursive;

use error::{self, TgTuiError};


pub trait Command: Sized {
    fn execute(&self, siv: &mut Cursive) -> error::Result<()> {
        // Trait docs are nicer with "siv" than "_siv"
        let _ = siv;

        Ok(())
    }

    fn try_from_str<'a, I>(command_name: &str, args: I) -> error::Result<Self>
        where I: IntoIterator<Item=&'a str>
    {
        // Trait docs are nicer with "args" than "_args"
        let _ = args;

        bail_err!(TgTuiError::UndefinedCommand { cmd: command_name.to_owned() })
    }
}


#[derive(Debug, PartialEq)]
pub struct CommandImpl {
    kind: CommandKind,
    args: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CommandKind {
    Info, Write, Quit,
}

impl Command for CommandImpl {
    fn execute(&self, siv: &mut Cursive) -> error::Result<()> {
        match self.kind {
            CommandKind::Quit => siv.quit(),
            _ => { /*  TODO: implement execution for other kinds */ },
        }

        Ok(())
    }

    fn try_from_str<'a, I>(command_name: &str, args: I) -> error::Result<CommandImpl>
        where I: IntoIterator<Item=&'a str>
    {
        let kind = match command_name {
            "info" => CommandKind::Info,
            "write" | "w" => CommandKind::Write,
            "quit" | "q" => CommandKind::Quit,

            _ => bail_err!(TgTuiError::UndefinedCommand { cmd: command_name.to_owned() }),
        };

        Ok(CommandImpl {
            kind: kind,
            args: collect_args(args),
        })
    }
}

impl Command for Vec<CommandImpl> {
    fn execute(&self, siv: &mut Cursive) -> error::Result<()> {
        for cmd in self {
            cmd.execute(siv)?;
        }

        Ok(())
    }

    fn try_from_str<'a, I>(command_name: &str, args: I) -> error::Result<Vec<CommandImpl>>
        where I: IntoIterator<Item=&'a str>
    {
        let commands = match CommandImpl::try_from_str(command_name, args) {
            Ok(cmd) => vec![cmd],
            Err(TgTuiError::UndefinedCommand { cmd: cmd_name }) => {
                assert_eq!(cmd_name, command_name);

                match command_name {
                    "wq" => compound_command(&[CommandKind::Write, CommandKind::Quit]),

                    _ => bail_err!(TgTuiError::UndefinedCommand { cmd: cmd_name }),
                }
            },
            Err(other_error) => bail_err!(other_error),
        };

        Ok(commands)
    }
}

fn compound_command<'a, I>(kinds: I) -> Vec<CommandImpl>
    where I: IntoIterator<Item=&'a CommandKind>
{
    kinds.into_iter()
        .map(|k| CommandImpl {
            kind: k.clone(),
            args: vec![],
        })
        .collect()
}

fn collect_args<'a, I>(args: I) -> Vec<String>
    where I: IntoIterator<Item=&'a str>
{
    args.into_iter().map(String::from).collect()
}


#[cfg(test)]
mod tests {
    use super::{CommandImpl, CommandKind, parse_command};

    #[test]
    fn test_single_command() {
        let input = ":write foo    bar";
        let parsed = parse_command::<CommandImpl>(input).unwrap();
        let expected = CommandImpl {
            kind: CommandKind::Write,
            args: vec!["foo".to_owned(), "bar".to_owned()],
        };

        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_single_command2() {
        let input = ":q";
        let parsed = parse_command::<CommandImpl>(input).unwrap();
        let expected = CommandImpl {
            kind: CommandKind::Quit,
            args: vec![],
        };

        assert_eq!(parsed, expected);
    }

    #[test]
    fn test_compound_command() {
        let input = ":wq";
        let parsed = parse_command::<Vec<CommandImpl>>(input).unwrap();
        let expected = vec![
            CommandImpl {
                kind: CommandKind::Write,
                args: vec![],
            },
            CommandImpl {
                kind: CommandKind::Quit,
                args: vec![],
            },
        ];

        assert_eq!(parsed, expected);
    }
}
