use std::thread;

use cursive::Cursive;
use rand::{self, Rng};

use action::Action;
use cursive_views::actions_view::ActionsView;


pub fn install_async_handlers(siv: &mut Cursive) {
    let tx = siv.cb_sink().clone();
    let callback = Box::new(add_data);

    thread::spawn(move || {
        tx.send(callback).expect("channel disconnected");
    });
}

fn add_data(siv: &mut Cursive) {
    let mut rng = rand::thread_rng();

    let nickname_len = rng.gen_range(0, 20);
    let nickname = rng
        .gen_ascii_chars()
        .take(nickname_len)
        .collect::<String>();

    let text_len = rng.gen_range(0, 150);
    let text = rng
        .gen_ascii_chars()
        .take(text_len)
        .collect::<String>();

    siv.call_on_id("actions-view", |view: &mut ActionsView| {
        view.add_action(Action::message(&nickname, &text));
    });
}
