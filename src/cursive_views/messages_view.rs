use std::cmp;

use cursive::{Printer, With};
use cursive::vec::Vec2;
use cursive::view::{ScrollBase, View};
use textwrap;

use common::Action;
use error;
use utils::strtime;


pub struct MessagesView {
    children: Vec<MessagesViewChild>,

    scrollbase: ScrollBase,
    focus: usize,
    rows: Option<Vec<String>>,
    needs_relayout: bool,
}

impl MessagesView {
    pub fn new() -> Self {
        MessagesView {
            children: Vec::new(),
            scrollbase: ScrollBase::new(),
            focus: 0,
            rows: None,
            needs_relayout: false,
        }
    }

    pub fn add_action(&mut self, action: Action) {
        let msgs_view_child = MessagesViewChild::Action(action);

        self.needs_relayout = true;
        self.children.push(msgs_view_child);
    }

    pub fn action(self, action: Action) -> Self {
        self.with(|s| s.add_action(action))
    }

    pub fn add_delimiter(&mut self) {
        self.children.push(MessagesViewChild::Delimiter);
    }

    pub fn delimiter(self) -> Self {
        self.with(|s| s.add_delimiter())
    }

    fn compute_rows(&mut self, available_size: &Vec2) -> error::Result<()> {
        let max_second_column_width = self.children.iter()
            .map(|msgs_view_child| match *msgs_view_child {
                MessagesViewChild::Action(ref action) => match *action {
                    Action::Online { ref username, .. } => username.len(),
                    Action::Offline { ref username, .. } => username.len(),
                    Action::Message { ref username, .. } => username.len(),
                },
                MessagesViewChild::Delimiter => 0,
            })
            .max()
            .map(|m| cmp::max(m, 3))    // 3 for "-->" and "<--"
            .unwrap_or(3);

        let msg_content_width = available_size.x
            .saturating_sub(8)     // "%H:%M:%S" time
            .saturating_sub(1)     // a space
            .saturating_sub(max_second_column_width)
            .saturating_sub(3);    // " | "

        let mut new_rows = Vec::new();

        for msgs_view_child in self.children.iter() {
            match *msgs_view_child {
                MessagesViewChild::Action(ref action) => match *action {
                    Action::Online { ref time, ref username } => {
                        new_rows.push(format!("{} {:>width$} | {} is online",
                            strtime(time)?, "-->", username, width=max_second_column_width));
                    },
                    Action::Offline { ref time, ref username } => {
                        new_rows.push(format!("{} {:>width$} | {} went offline",
                            strtime(time)?, "<--", username, width=max_second_column_width));
                    },
                    Action::Message { ref time, ref username, ref text } => {
                        let stime = strtime(time)?;

                        for (i, msg_content_row) in textwrap::wrap(text, msg_content_width).into_iter().enumerate() {
                            let fmt_row = if i == 0 {
                                format!("{} {:>width$} | {}",
                                    stime, username, msg_content_row, width=max_second_column_width)
                            } else {
                                format!("{:8} {:>width$} | {}",
                                    "", "", msg_content_row, width=max_second_column_width)
                            };

                            new_rows.push(fmt_row);
                        }
                    },
                },
                MessagesViewChild::Delimiter => {
                    new_rows.push(format!("{:->width$}", "", width=available_size.x))
                }
            }
        }

        self.rows = Some(new_rows);

        Ok(())
    }
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
        if self.needs_relayout {
            self.compute_rows(&size).unwrap();
            self.needs_relayout = false;
        }

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
