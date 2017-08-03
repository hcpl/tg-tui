use std::cmp;

use cursive::{Printer, With};
use cursive::vec::Vec2;
use cursive::view::{ScrollBase, View};
use time;

use common::Action;
use error;


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

        let new_rows = self.children.iter()
            .map(|msgs_view_child| -> error::Result<String> {
                let row = match *msgs_view_child {
                    MessagesViewChild::Action(ref action) => match *action {
                        Action::Online { ref time, ref username } => {
                            format!("{} {:>width$} | {} is online",
                                strtime(time)?, "-->", username, width=max_second_column_width)
                        },
                        Action::Offline { ref time, ref username } => {
                            format!("{} {:>width$} | {} went offline",
                                strtime(time)?, "<--", username, width=max_second_column_width)
                        },
                        Action::Message { ref time, ref username, ref text } => {
                            format!("{} {:>width$} | {}",
                                strtime(time)?, username, text, width=max_second_column_width)
                        },
                    },
                    MessagesViewChild::Delimiter => {
                        format!("{:->width$}", "", width=available_size.x)
                    }
                };

                Ok(row)
            })
            .collect::<error::Result<Vec<_>>>()?;

        self.rows = Some(new_rows);

        Ok(())
    }
}

fn strtime(time: &time::Tm) -> error::Result<String> {
    time::strftime("%H:%M:%S", time).map_err(Into::into)
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
