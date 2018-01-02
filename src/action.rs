use chrono::prelude::{DateTime, Local};


#[derive(Debug)]
pub enum Action {
    Online {
        date_time: DateTime<Local>,
        username: String,
    },
    Offline {
        date_time: DateTime<Local>,
        username: String,
    },
    Message {
        date_time: DateTime<Local>,
        username: String,
        text: String,
    },
    SelfConnect {
        date_time: DateTime<Local>,
    },
    SelfDisconnect {
        date_time: DateTime<Local>,
    },
    CommandOutput {
        date_time: DateTime<Local>,
        command: String,
        output: String,
    },
}

impl Action {
    // A bunch of conventional methods to create actions without having to know what is the current
    // time at the moment of calling them.

    pub fn online(username: &str) -> Action {
        Action::Online {
            date_time: Local::now(),
            username: username.to_owned(),
        }
    }

    pub fn offline(username: &str) -> Action {
        Action::Offline {
            date_time: Local::now(),
            username: username.to_owned(),
        }
    }

    pub fn message(username: &str, text: &str) -> Action {
        Action::Message {
            date_time: Local::now(),
            username: username.to_owned(),
            text: text.to_owned(),
        }
    }

    pub fn self_connect() -> Action {
        Action::SelfConnect {
            date_time: Local::now(),
        }
    }

    pub fn self_disconnect() -> Action {
        Action::SelfDisconnect {
            date_time: Local::now(),
        }
    }

    pub fn command_output(command: &str, output: &str) -> Action {
        Action::CommandOutput {
            date_time: Local::now(),
            command: command.to_owned(),
            output: output.to_owned(),
        }
    }

    // Methods to get specific data from an action without having to match all variants every time
    // you need only one field.

    pub fn date_time(&self) -> Option<&DateTime<Local>> {
        let date_time = match *self {
            Action::Online { ref date_time, .. } => date_time,
            Action::Offline { ref date_time, .. } => date_time,
            Action::Message { ref date_time, .. } => date_time,
            Action::SelfConnect { ref date_time, .. } => date_time,
            Action::SelfDisconnect { ref date_time, .. } => date_time,
            Action::CommandOutput { ref date_time, .. } => date_time,
        };

        Some(date_time)
    }

    pub fn username(&self) -> Option<&str> {
        let username = match *self {
            Action::Online { ref username, .. } => username,
            Action::Offline { ref username, .. } => username,
            Action::Message { ref username, .. } => username,
            _ => return None,
        };

        Some(username)
    }
}
