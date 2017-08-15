use cursive::align::HAlign;
use cursive::event::{Event, Key};
use cursive::view::{Identifiable, ViewWrapper};
use cursive::views::{BoxView, IdView, LinearLayout, OnEventView, TextArea, TextView};
use time;

use common::Action;
use cursive_views::messages_view::MessagesView;
use error;
use utils;


pub struct Dialog {
    layout: LinearLayout,
}

impl Dialog {
    pub fn new() -> error::Result<Dialog> {
        let layout = LinearLayout::vertical()
            .child(Dialog::create_dialog_title())
            .child(Dialog::create_messages_display_area())
            .child(Dialog::create_status_bar()?)
            .child(Dialog::create_message_edit_area());

        Ok(Dialog {
            layout: layout,
        })
    }

    fn create_dialog_title() -> IdView<TextView> {
        let dialog_title = TextView::new("dialog title")
            .h_align(HAlign::Center)
            .with_id("dialog_title");

        dialog_title
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
            .action(Action::Message {
                time: time::now(),
                username: "".to_owned(),
                text: "Lorem ipsum dolor sit amet, consectetur adipiscing elit, \
                       sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. \
                       Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris \
                       nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in \
                       reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla \
                       pariatur. Excepteur sint occaecat cupidatat non proident, sunt in \
                       culpa qui officia deserunt mollit anim id est laborum.".to_owned(),
            })
            .delimiter()
            .with_id("messages_view"))
    }

    fn create_status_bar() -> error::Result<IdView<TextView>> {
        let status_bar = TextView::new(utils::strnow()?).with_id("status_bar");

        Ok(status_bar)
    }

    fn create_message_edit_area() -> LinearLayout {
        let prompt = "prompt";
        let initial_message_text = "message text";

        let message_edit_area = LinearLayout::horizontal()
            .child(TextView::new(prompt))
            .child(BoxView::with_full_width(
                OnEventView::new(TextArea::new().content(initial_message_text).with_id("message-edit"))
                    .on_event(Event::Ctrl(Key::Enter), |s| {
                        s.call_on_id("message-edit", |v: &mut IdView<TextArea>| {
                            v.get_mut().set_content("");
                        });
                    })));

        message_edit_area
    }
}

impl ViewWrapper for Dialog {
    wrap_impl!(self.layout: LinearLayout);
}
