use std::collections::HashMap;

use cursive::Cursive;

use error::{self, ErrorKind};
use mode::Mode;


type Callback = Box<Fn(&mut Cursive)>;
type ModeBindings = HashMap<&'static str, Callback>;

struct Bindings {
    bindings: HashMap<Mode, ModeBindings>,
}

impl Bindings {
    /// Default bindings
    fn new() -> Bindings {
        let mut normal: ModeBindings = HashMap::new();
        normal.insert("a", Box::new(|siv| unimplemented!()));
        normal.insert("d", Box::new(|siv| unimplemented!()));
        normal.insert("i", Box::new(|siv| unimplemented!()));
        normal.insert("q", Box::new(|siv| siv.quit()));
        normal.insert("s", Box::new(|siv| unimplemented!()));
        normal.insert("u", Box::new(|siv| unimplemented!()));

        let mut visual: ModeBindings = HashMap::new();
        visual.insert("a", Box::new(|siv| unimplemented!()));
        visual.insert("d", Box::new(|siv| unimplemented!()));

        let mut bindings = HashMap::new();
        bindings.insert(Mode::Normal, normal);
        bindings.insert(Mode::Visual, visual);

        Bindings {
            bindings: bindings,
        }
    }

    fn get(&self, mode: Mode, binding: &'static str) -> error::Result<&Callback> {
        let mode_bindings = self.bindings.get(&mode)
            .ok_or(ErrorKind::BindingModeNonRegisterable(mode))?;
        let callback = mode_bindings.get(binding)
            .ok_or(ErrorKind::BindingNotFound(mode, binding))?;

        Ok(callback)
    }

    fn set(&mut self, mode: Mode, binding: &'static str, callback: Callback) -> error::Result<()> {
        let mode_bindings = self.bindings.get_mut(&mode)
            .ok_or(ErrorKind::BindingModeNonRegisterable(mode))?;
        mode_bindings.insert(binding, callback);

        Ok(())
    }
}

impl Default for Bindings {
    fn default() -> Bindings {
        Bindings::new()
    }
}
