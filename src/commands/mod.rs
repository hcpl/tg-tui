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


fn collect_args<'a, I>(args: I) -> Vec<String>
    where I: IntoIterator<Item=&'a str>
{
    args.into_iter().map(String::from).collect()
}

pub enum CommandImpl {
    Info(Vec<String>),
    Write(Vec<String>),
}

impl Command for CommandImpl {
    fn execute(&self) -> error::Result<()> {
        // TODO: implement execution
        Ok(())
    }

    fn try_from_str<'a, I>(command_name: &str, args: I) -> error::Result<CommandImpl>
        where I: IntoIterator<Item=&'a str>
    {
        let cmd_fn = match command_name {
            "info" => CommandImpl::Info,
            "write" | "w" => CommandImpl::Write,
            _ => bail!(ErrorKind::UndefinedCommand(command_name.to_owned())),
        };

        Ok(cmd_fn(collect_args(args)))
    }
}
