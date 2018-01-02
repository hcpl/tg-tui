use std::cmp;

use cursive::{Printer, With};
use cursive::vec::Vec2;
use cursive::view::{ScrollBase, View};
use textwrap;

use action::Action;
use error;
use utils::strtime;


/// A Cursive view for displaying user actions in default WeeChat style: date-time,
/// nickname of the user that triggered the action, and the action contents.
pub struct ActionsView {
    children: Vec<ActionsViewChild>,

    scrollbase: ScrollBase,
    focus: usize,
    rows: Option<Vec<String>>,
    needs_relayout: bool,
}

impl ActionsView {
    /// Create an empty message view.
    pub fn new() -> Self {
        ActionsView {
            children: Vec::new(),

            scrollbase: ScrollBase::new(),
            focus: 0,
            rows: None,
            needs_relayout: false,
        }
    }

    /// Add a `Action` to the message stream.
    pub fn add_action(&mut self, action: Action) {
        let msgs_view_child = ActionsViewChild::Action(action);

        self.needs_relayout = true;
        self.children.push(msgs_view_child);
    }

    /// The chaining version of `add_action()`.
    pub fn action(self, action: Action) -> Self {
        self.with(|s| s.add_action(action))
    }

    /// Add a delimiter, useful to mark actions below it as unseen.
    pub fn add_delimiter(&mut self) {
        self.children.push(ActionsViewChild::Delimiter);
    }

    /// The chaining version of `add_delimiter()`.
    pub fn delimiter(self) -> Self {
        self.with(|s| s.add_delimiter())
    }

    // FIXME: Do computations lazily!
    /// Compute element dimensions and positions to draw contents of this
    /// `ActionsView`.
    fn compute_rows(&mut self, available_size: &Vec2) -> error::Result<()> {
        let max_second_column_width = self.children.iter()
            .map(|msgs_view_child| match *msgs_view_child {
                ActionsViewChild::Action(ref action) =>
                    action.username().map(str::len).unwrap_or(0),
                ActionsViewChild::Delimiter => 0,
            })
            .max()
            .map(|m| cmp::max(m, 3))    // 3 for "-->" and "<--"
            .unwrap_or(3);

        let content_width = available_size.x
            .saturating_sub(8)     // "%H:%M:%S" time
            .saturating_sub(1)     // a space
            .saturating_sub(max_second_column_width)
            .saturating_sub(3);    // " | "

        let mut new_rows = Vec::new();

        for msgs_view_child in self.children.iter() {
            match *msgs_view_child {
                ActionsViewChild::Action(ref action) => match *action {
                    Action::Online { ref date_time, ref username } => {
                        new_rows.push(format!("{} {:>width$} | {} is online",
                            strtime(date_time), "-->", username, width=max_second_column_width));
                    },
                    Action::Offline { ref date_time, ref username } => {
                        new_rows.push(format!("{} {:>width$} | {} went offline",
                            strtime(date_time), "<--", username, width=max_second_column_width));
                    },
                    Action::Message { ref date_time, ref username, ref text } => {
                        let stime = strtime(date_time);

                        for (i, msg_content_row) in textwrap::wrap_iter(text, content_width).enumerate() {
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
                    Action::SelfConnect { ref date_time } => {
                        new_rows.push(format!("{} {:>width$} | {}",
                            strtime(date_time), "--", "telegram: connected to server", width=max_second_column_width));
                    },
                    Action::SelfDisconnect { ref date_time } => {
                        new_rows.push(format!("{} {:>width$} | {}",
                            strtime(date_time), "--", "telegram: disconnected from server", width=max_second_column_width));
                    },
                    Action::CommandOutput { ref date_time, ref command, ref output } => {
                        let stime = strtime(date_time);

                        new_rows.push(format!("{} {:>width$} | {}",
                            strtime(date_time), "", "", width=max_second_column_width));


                        for (i, cmd_content_row) in textwrap::wrap_iter(output, content_width).enumerate() {
                            let fmt_row = if i == 0 {
                                format!("{} {:>width$} | {}",
                                    stime, "", cmd_content_row, width=max_second_column_width)
                            } else {
                                format!("{:8} {:>width$} | {}",
                                    "", "", cmd_content_row, width=max_second_column_width)
                            };

                            new_rows.push(fmt_row);
                        }
                    },
                },
                ActionsViewChild::Delimiter => {
                    new_rows.push(format!("{:->width$}", "", width=available_size.x))
                },
            }
        }

        self.rows = Some(new_rows);

        Ok(())
    }
}

impl View for ActionsView {
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
            self.compute_rows(&size).unwrap(); // FIXME: Deal with Results here more gracefully
            self.needs_relayout = false;
        }

        let rows_len = match self.rows {
            Some(ref rows) => rows.len(),
            None           => 0,
        };

        self.scrollbase.set_heights(size.y, rows_len);
    }
}


pub enum ActionsViewChild {
    Action(Action),
    Delimiter,
}
