use cursive::view::Identifiable;
use cursive::views::{BoxView, TextArea, TextView};
use time;

use common::Action;
use cursive_views::messages_view::MessagesView;
use error;
use utils;


pub fn create_main_layout() -> error::Result<LinearLayout> {
    let layout = LinearLayout::vertical()
        .child(create_messages_display_area())
        .child(create_status_bar()?)
        .child(create_message_edit_area());

    Ok(layout)
}

fn create_messages_display_area() -> BoxView<IdView<MessagesView>> {
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
        .delimiter()
        .with_id("messages_view"))
}

fn create_status_bar() -> error::Result<IdView<TextView>> {
    let status_bar = TextView::new(utils::now()?).with_id("status_bar");

    Ok(status_bar)
}

fn create_message_edit_area() -> LinearLayout {
    let prompt = "prompt";
    let initial_message_text = "message text";

    let message_edit_area = LinearLayout::horizontal()
        .child(TextView::new(prompt))
        .child(BoxView::with_full_width(TextArea::new().content(initial_message_text)));

    message_edit_area
}
