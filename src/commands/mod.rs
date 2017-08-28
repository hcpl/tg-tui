pub mod parser;

pub use self::parser::parse_command;

use error::{self, ErrorKind};


pub trait Command: Sized {
    fn execute(&self) -> error::Result<()> {
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

pub enum CommandKind {
    Info, Write, Quit,
}

impl Command for CommandImpl {
    fn execute(&self) -> error::Result<()> {
        // TODO: implement execution
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
            args: collect_args(args)
        })
    }
}

fn collect_args<'a, I>(args: I) -> Vec<String>
    where I: IntoIterator<Item=&'a str>
{
    args.into_iter().map(String::from).collect()
}
