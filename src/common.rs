use time::Tm;


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
