mod parser;

use error;


trait Command {
    fn execute(args: &[&str]) -> error::Result<()>;
}


struct Info;

impl Command for Info {
    fn execute(_args: &[&str]) -> error::Result<()> {
        // TODO: implement info gathering
        Ok(())
    }
}
