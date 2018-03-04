mod dialog;


use cursive::Cursive;
use cursive::event::Event;
use cursive::theme::{BorderStyle, BaseColor, Color, PaletteColor, Theme};
use cursive::view::Identifiable;
use cursive::views::{BoxView, Dialog, EditView, IdView, TextView};

use app_config::AppConfig;
use error;
use utils;


pub fn create_cursive_session(app_config: &mut AppConfig) -> error::Result<Cursive> {
    let mut siv = Cursive::new();

    siv.set_theme(custom_theme());
    siv.add_global_callback('q', |s| s.quit());
    siv.add_fullscreen_layer(BoxView::with_full_screen(dialog::Dialog::new()?));

    match app_config.phone_number {
        // If there is no phone number in config, request it from the user.
        None => siv.add_layer(create_authorization_dialog(app_config)),
        // Otherwise, there is a phone number already in config, so no need to ask for it again.
        Some(_) => {},
    }

    siv.set_fps(1);
    siv.add_global_callback(Event::Refresh, |s| {
        s.call_on_id("status-bar", |v: &mut IdView<TextView>| {
            v.get_mut().set_content(utils::local_strnow());
        });
    });

    Ok(siv)
}


fn custom_theme() -> Theme {
    Theme {
        shadow: false,
        borders: BorderStyle::Simple,
        palette: enum_map! {
            PaletteColor::Background => Color::TerminalDefault,
            PaletteColor::Shadow => Color::Dark(BaseColor::Black),
            PaletteColor::View => Color::TerminalDefault,
            PaletteColor::Primary => Color::Dark(BaseColor::White),
            PaletteColor::Secondary => Color::Dark(BaseColor::Cyan),
            PaletteColor::Tertiary => Color::Light(BaseColor::Magenta),
            PaletteColor::TitlePrimary => Color::Dark(BaseColor::Red),
            PaletteColor::TitleSecondary => Color::Dark(BaseColor::Yellow),
            PaletteColor::Highlight => Color::Dark(BaseColor::Green),
            PaletteColor::HighlightInactive => Color::Dark(BaseColor::Blue),
        },
    }
}

fn create_authorization_dialog(app_config: &mut AppConfig) -> Dialog {
    Dialog::new()
        .title("Enter your phone number")
        .padding((1, 1, 1, 0))
        .content(EditView::new().with_id("phone-number-edit"))
        .button("Ok", |s| {
            /*s.call_on_id("phone-number-edit", move |v: &mut IdView<EditView>| {
                let number = String::from(*v.get_mut().get_content());
                //app_config.phone_number = Some(number);
            });*/
            // TODO: send authorization request
            s.pop_layer();
        })
}
