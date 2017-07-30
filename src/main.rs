#[macro_use(wrap_impl)]
extern crate cursive;
extern crate time;

mod cursive_views;
mod common;

use cursive::Cursive;
use cursive::event::Event;
use cursive::theme::{BorderStyle, BaseColor, Color, Palette, Theme};
use cursive::view::Identifiable;
use cursive::views::{BoxView, Dialog, EditView, IdView, LinearLayout, TextArea, TextView};

use common::Action;
use cursive_views::messages_view::MessagesView;

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

fn now() -> String {
    time::strftime("%H:%M:%S", &time::now()).unwrap()
}


fn create_messages_display_area() -> BoxView<MessagesView> {
    BoxView::with_full_screen(MessagesView::new()
        .action(Action::Online {
            time: time::now(),
            username: "foo".to_owned(),
        })
        .action(Action::Offline {
            time: time::now(),
            username: "bar".to_owned(),
        })
        .action(Action::Message {
            time: time::now(),
            username: "deadbeef".to_owned(),
            text: "hello tg-tui from deadbeef".to_owned(),
        })
        .delimiter())
}

fn create_status_bar() -> IdView<TextView> {
    let status_bar = TextView::new(now()).with_id("status_bar");

    status_bar
}

fn create_message_edit_area() -> LinearLayout {
    let message_edit_area = LinearLayout::horizontal();

    let initial_message_text = "message text";

    message_edit_area
        .child(BoxView::with_full_width(TextArea::new().content(initial_message_text)))
}

fn create_main_layout() -> LinearLayout {
    let layout = LinearLayout::vertical();

    layout
        .child(create_messages_display_area())
        .child(create_status_bar())
        .child(create_message_edit_area())
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


fn main() {
    let mut siv = Cursive::new();

    siv.set_theme(custom_theme());
    siv.add_global_callback('q', |s| s.quit());
    siv.add_fullscreen_layer(BoxView::with_full_screen(create_main_layout()));
    siv.add_layer(create_authorization_dialog());

    siv.set_fps(1);
    siv.add_global_callback(Event::Refresh, |s| {
        s.call_on_id("status_bar", |v: &mut IdView<TextView>| {
            v.get_mut().set_content(now());
        });
    });

    siv.run();
}
