use cursive::{Printer, With};
use cursive::direction::Direction;
use cursive::vec::Vec2;
use cursive::view::{ScrollBase, View};
use time;

use common::Action;


pub struct MessagesView {
    children: Vec<MessagesViewChild>,

    scrollbase: ScrollBase,
    focus: usize,
    rows: Option<Vec<String>>,
}

impl MessagesView {
    pub fn new() -> Self {
        MessagesView {
            children: Vec::new(),
            scrollbase: ScrollBase::new(),
            focus: 0,
            rows: None,
        }
    }

    pub fn add_action(&mut self, action: Action) {
        let msgs_view_child = MessagesViewChild::Action(action);

        self.add_rows(&msgs_view_child);
        self.children.push(msgs_view_child);
    }

    pub fn action(self, action: Action) -> Self {
        self.with(|s| s.add_action(action))
    }

    fn add_rows(&mut self, msgs_view_child: &MessagesViewChild) {
        let new_rows = match *msgs_view_child {
            MessagesViewChild::Action(ref action) => match *action {
                Action::Online { ref time, ref username } => {
                    format!("{} --> | {} is online", strtime(time), username)
                },
                Action::Offline { ref time, ref username } => {
                    format!("{} <-- | {} went offline", strtime(time), username)
                },
                Action::Message { ref time, ref username, ref text } => {
                    format!("{} {} | {}", strtime(time), username, text)
                },
            },
            MessagesViewChild::Delimiter => "".to_owned(),
        };

        if let Some(ref mut rows) = self.rows {
            rows.push(new_rows);
        } else {
            self.rows = Some(vec![new_rows]);
        }
    }
}

fn strtime(time: &time::Tm) -> String {
    time::strftime("%H:%M:%S", time).unwrap()
}

impl View for MessagesView {
    fn draw(&self, printer: &Printer) {
        self.scrollbase.draw(printer, |printer, i| {
            if let Some(ref rows) = self.rows {
                if let Some(ref line) = rows.get(i) {
                    printer.print((0,0), line)
                }
            }
        });
    }

    fn layout(&mut self, size: Vec2) {
        let rows_len = match self.rows {
            Some(ref rows) => rows.len(),
            None           => 0,
        };

        self.scrollbase.set_heights(size.y, rows_len);
    }
}


pub enum MessagesViewChild {
    Action(Action),
    Delimiter,
}
