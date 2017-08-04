mod dialog;

use cursive::Cursive;
use cursive::event::Event;
use cursive::theme::{BorderStyle, BaseColor, Color, Palette, Theme};
use cursive::views::{BoxView, Dialog, EditView, IdView, TextView};

use error;
use utils;


pub fn create_cursive_session() -> error::Result<Cursive> {
    let mut siv = Cursive::new();

    siv.set_theme(custom_theme());
    siv.add_global_callback('q', |s| s.quit());
    siv.add_fullscreen_layer(BoxView::with_full_screen(dialog::create_main_layout()?));
    siv.add_layer(create_authorization_dialog());

    siv.set_fps(1);
    siv.add_global_callback(Event::Refresh, |s| {
        s.call_on_id("status_bar", |v: &mut IdView<TextView>| {
            v.get_mut().set_content(utils::now().unwrap());
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
