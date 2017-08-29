use std::collections::HashMap;

use cursive::Cursive;

use error::{self, ErrorKind};
use mode::Mode;


type Callback = Box<Fn(&mut Cursive) + Sync>;
type CallbacksNames = HashMap<String, Callback>;
type ModeBindings = HashMap<String, String>;

lazy_static! {
    static ref CALLBACKS_NAMES: HashMap<String, Callback> = {
        let mut cb_names: CallbacksNames = HashMap::new();

        cb_names.insert("quit".to_owned(), Box::new(|siv| siv.quit()));

        cb_names
    };
}

#[derive(Deserialize)]
pub struct Bindings {
    bindings: HashMap<Mode, ModeBindings>,
}

impl Bindings {
    /// Default bindings
    fn new() -> Bindings {
        let mut normal: ModeBindings = HashMap::new();
        normal.insert("a".to_owned(), "append_after_cursor".to_owned());
        //normal.insert("d", "delete");
        normal.insert("i".to_owned(), "insert".to_owned());
        normal.insert("q".to_owned(), "quit".to_owned());
        //normal.insert("s", "");
        normal.insert("u".to_owned(), "undo".to_owned());

        let mut visual: ModeBindings = HashMap::new();
        visual.insert("a".to_owned(), "append_after_cursor".to_owned());
        visual.insert("d".to_owned(), "delete".to_owned());

        let mut bindings = HashMap::new();
        bindings.insert(Mode::Normal, normal);
        bindings.insert(Mode::Visual, visual);

        Bindings {
            bindings: bindings,
        }
    }

    fn get(&self, mode: Mode, binding: &str) -> error::Result<&str> {
        let mode_bindings = self.bindings.get(&mode)
            .ok_or(ErrorKind::BindingModeNonRegisterable(mode))?;
        let callback_name = mode_bindings.get(binding)
            .ok_or(ErrorKind::BindingNotFound(mode, binding.to_owned()))?;

        Ok(callback_name)
    }

    fn set(&mut self, mode: Mode, binding: &str, callback_name: &str) -> error::Result<()> {
        let mode_bindings = self.bindings.get_mut(&mode)
            .ok_or(ErrorKind::BindingModeNonRegisterable(mode))?;
        mode_bindings.insert(binding.to_owned(), callback_name.to_owned());

        Ok(())
    }
}

impl Default for Bindings {
    fn default() -> Bindings {
        Bindings::new()
    }
}
