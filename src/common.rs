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
}

impl Action {
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
}
