use time::{self, Tm};


#[derive(Debug)]
pub enum Action {
    Online {
        time: Tm,
        username: String,
    },
    Offline {
        time: Tm,
        username: String,
    },
    Message {
        time: Tm,
        username: String,
        text: String,
    },
}

impl Action {
    pub fn online(username: &str) -> Action {
        Action::Online {
            time: time::now(),
            username: username.to_owned(),
        }
    }

    pub fn offline(username: &str) -> Action {
        Action::Offline {
            time: time::now(),
            username: username.to_owned(),
        }
    }

    pub fn message(username: &str, text: &str) -> Action {
        Action::Message {
            time: time::now(),
            username: username.to_owned(),
            text: text.to_owned(),
        }
    }
}
