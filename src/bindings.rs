use std::collections::HashMap;
use std::ops;
use std::sync::Arc;

use cursive::Cursive;

use error::{self, ErrorKind};
use mode::Mode;


type SimpleCallback = Fn(&mut Cursive) -> error::Result<()> + Sync + Send;
type BoxedSimpleCallback = Box<SimpleCallback>;

struct Callback(Arc<BoxedSimpleCallback>);

impl Callback {
    fn from_fn<F>(f: F) -> Callback
        where F: Fn(&mut Cursive) -> error::Result<()> + Sync + Send + 'static
    {
        Callback(Arc::new(Box::new(f)))
    }

    fn from_non_result_fn<F>(f: F) -> Callback
        where F: Fn(&mut Cursive) + Sync + Send + 'static
    {
        Callback::from_fn(move |siv| {
            f(siv);

            Ok(())
        })
    }
}

impl ops::Deref for Callback {
    type Target = BoxedSimpleCallback;

    fn deref<'a>(&'a self) -> &'a BoxedSimpleCallback {
        &self.0
    }
}


type CallbacksNames = HashMap<String, Callback>;

lazy_static! {
    static ref CALLBACKS_NAMES: CallbacksNames = {
        let mut cb_names: CallbacksNames = HashMap::new();

        cb_names.insert("quit".to_owned(), Callback::from_non_result_fn(|siv| siv.quit()));

        cb_names
    };
}


type ModeBindings = HashMap<String, String>;

#[derive(Debug, Deserialize)]
pub struct Bindings {
    bindings: HashMap<Mode, ModeBindings>,
}

impl Bindings {
    /// Default bindings
    pub fn new() -> Bindings {
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

    pub fn get(&self, mode: Mode, binding: &str) -> error::Result<&str> {
        let mode_bindings = self.bindings.get(&mode)
            .ok_or(ErrorKind::BindingModeNonRegisterable(mode))?;
        let callback_name = mode_bindings.get(binding)
            .ok_or(ErrorKind::BindingNotFound(mode, binding.to_owned()))?;

        Ok(callback_name)
    }

    pub fn get_callback(&self, mode: Mode, binding: &str) -> error::Result<&BoxedSimpleCallback> {
        let callback_name = self.get(mode, binding)?;
        let callback = CALLBACKS_NAMES.get(callback_name)
            .ok_or(ErrorKind::InvalidCallbackName(callback_name.to_owned()))?;

        Ok(&**callback)
    }

    pub fn set(&mut self, mode: Mode, binding: &str, callback_name: &str) -> error::Result<()> {
        let mode_bindings = self.bindings.get_mut(&mode)
            .ok_or(ErrorKind::BindingModeNonRegisterable(mode))?;
        mode_bindings.insert(binding.to_owned(), callback_name.to_owned());

        Ok(())
    }

    pub fn execute(&self, mode: Mode, binding: &str, siv: &mut Cursive) -> error::Result<()> {
        let callback = self.get_callback(mode, binding)?;

        callback(siv)?;

        Ok(())
    }
}

impl Default for Bindings {
    fn default() -> Bindings {
        Bindings::new()
    }
}
