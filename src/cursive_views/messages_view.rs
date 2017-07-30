use cursive::{Printer, With};
use cursive::vec::Vec2;
use cursive::view::{ScrollBase, View};

use common::Action;


pub struct MessagesView {
    children: Vec<MessagesViewChild>,
    scrollbase: ScrollBase,
    focus: usize,
}

impl MessagesView {
    pub fn new() -> Self {
        MessagesView {
            children: Vec::new(),
            scrollbase: ScrollBase::new(),
            focus: 0,
        }
    }

    pub fn add_action(&mut self, action: Action) {
        self.children.push(MessagesViewChild::Action(action));
    }

    pub fn action(self, action: Action) -> Self {
        self.with(|s| s.add_action(action))
    }
}

impl View for MessagesView {
    fn draw(&self, printer: &Printer) {
        self.scrollbase.draw(printer, |printer, i| match self.children[i] {
            MessagesViewChild::Action(ref action) => match *action {
                Action::Online { time, ref username } => {
                    //printer.print((0,0), &time.to_string());
                    printer.print((0,0), &username.to_string());
                },
                Action::Offline { time, ref username } => {
                    //printer.print((0,0), &time.to_string());
                    printer.print((0,0), &username.to_string());
                },
                Action::Message { time, ref username, ref text } => {
                    //printer.print((0,0), &time.to_string());
                    printer.print((0,0), &username.to_string());
                    printer.print((0,0), &text.to_string());
                },
            },
            MessagesViewChild::Delimiter => (),
        });
    }

    fn required_size(&mut self, req: Vec2) -> Vec2 {
        Vec2::new(20, 30)
    }
}


pub enum MessagesViewChild {
    Action(Action),
    Delimiter,
}
