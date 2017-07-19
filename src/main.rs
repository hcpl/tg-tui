#[macro_use(wrap_impl)]
extern crate cursive;
extern crate time;

//mod cursive_views;

use cursive::Cursive;
use cursive::event::Event;
use cursive::theme::{BorderStyle, BaseColor, Color, Palette, Theme};
use cursive::view::Identifiable;
use cursive::views::{BoxView, IdView, LinearLayout, TextArea, TextView};


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

fn create_status_bar() -> IdView<TextView> {
    let time = time::now();

    let status_bar = TextView::new(format!("{}", time.strftime("%H:%M:%S").unwrap())).with_id("status bar");

    status_bar
}

fn create_layout() -> LinearLayout {
    let layout = LinearLayout::vertical();

    layout
        .child(BoxView::with_full_screen(TextView::new("messages")))
        .child(create_status_bar())
        .child(TextArea::new().content("message text"))
}

fn main() {
    let mut siv = Cursive::new();

    siv.set_theme(custom_theme());
    siv.add_global_callback('q', |s| s.quit());
    //siv.add_fullscreen_layer(TextView::new("Hello cursive!"));
    //siv.add_fullscreen_layer(BoxView::with_full_screen(TextView::new("Hello cursive!")));
    siv.add_fullscreen_layer(BoxView::with_full_screen(create_layout()));

    siv.set_fps(1);
    siv.add_global_callback(Event::Refresh, |s| {
        s.call_on_id("status bar", |v: &mut IdView<TextView>| {
            v.get_mut().set_content(time::now().strftime("%H:%M:%S").unwrap().to_string());
        });
    });

    siv.run();
}
