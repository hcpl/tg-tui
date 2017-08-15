mod dialog;

use config::{Config, ConfigError};
use cursive::Cursive;
use cursive::event::Event;
use cursive::theme::{BorderStyle, BaseColor, Color, Palette, Theme};
use cursive::views::{BoxView, Dialog, EditView, IdView, TextView};

use error;
use utils;


pub fn create_cursive_session(config: &Config) -> error::Result<Cursive> {
    let mut siv = Cursive::new();

    siv.set_theme(custom_theme());
    siv.add_global_callback('q', |s| s.quit());
    siv.add_fullscreen_layer(BoxView::with_full_screen(dialog::Dialog::new()?));

    match config.get_str("phone-number") {
        // If there is no phone number already in config, request it from the user.
        Err(ConfigError::NotFound(_)) => siv.add_layer(create_authorization_dialog()),
        // If other error, propagate it.
        Err(e) => bail!(e),
        // Otherwise, there is a phone number in config, so no need to ask for it again.
        Ok(_) => {},
    }

    siv.set_fps(1);
    siv.add_global_callback(Event::Refresh, |s| {
        s.call_on_id("status_bar", |v: &mut IdView<TextView>| {
            v.get_mut().set_content(utils::strnow().unwrap());
        });
    });

    Ok(siv)
}


fn custom_theme() -> Theme {
    Theme {
        shadow: false,
        borders: BorderStyle::Simple,
        colors: Palette {
            background: Color::TerminalDefault,
            shadow: Color::Dark(BaseColor::Black),
            view: Color::TerminalDefault,
            primary: Color::Dark(BaseColor::White),
            secondary: Color::Dark(BaseColor::Cyan),
            tertiary: Color::Light(BaseColor::Magenta),
            title_primary: Color::Dark(BaseColor::Red),
            title_secondary: Color::Dark(BaseColor::Yellow),
            highlight: Color::Dark(BaseColor::Green),
            highlight_inactive: Color::Dark(BaseColor::Blue),
        }
    }
}

fn create_authorization_dialog() -> Dialog {
    Dialog::new()
        .title("Enter your phone number")
        .padding((1, 1, 1, 0))
        .content(EditView::new())
        .button("Ok", |s| {
            // TODO: send authorization request
            s.pop_layer();
        })
}
