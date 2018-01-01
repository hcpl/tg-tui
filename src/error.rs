use mode::Mode;


#[derive(Debug, Fail)]
pub enum TgTuiError {
    #[fail(display = "no bindings can be registered for {:?} mode", mode)]
    BindingModeNonRegisterable {
        mode: Mode,
    },

    #[fail(display = "no binding named {:?} found for {:?} mode", binding, mode)]
    BindingNotFound {
        mode: Mode,
        binding: String,
    },

    #[fail(display = "invalid callback name: {:?}", callback_name)]
    InvalidCallbackName {
        callback_name: String,
    },

    #[fail(display = "undefined command: {:?}", cmd)]
    UndefinedCommand {
        cmd: String,
    },
}

pub type Result<T> = ::std::result::Result<T, TgTuiError>;

macro_rules! bail_err {
    ($e:expr) => {{
        return Err($e.into());
    }};
}
