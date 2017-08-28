pub mod parser;

pub use self::parser::parse_command;


use cursive::Cursive;

use error::{self, ErrorKind};


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

        bail!(ErrorKind::UndefinedCommand(command_name.to_owned()))
    }
}


pub struct CommandImpl {
    kind: CommandKind,
    args: Vec<String>,
}

#[derive(Clone)]
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

            _ => bail!(ErrorKind::UndefinedCommand(command_name.to_owned())),
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
            Err(error::Error(ErrorKind::UndefinedCommand(cmd_name), _)) => {
                assert_eq!(cmd_name, command_name);

                match command_name {
                    "wq" => compound_command(&[CommandKind::Write, CommandKind::Quit]),

                    _ => bail!(ErrorKind::UndefinedCommand(cmd_name)),
                }
            },
            Err(other_error) => bail!(other_error),
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
