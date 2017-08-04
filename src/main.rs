extern crate app_dirs;
#[macro_use]
extern crate clap;
extern crate cursive;
#[macro_use]
extern crate error_chain;
extern crate rand;
extern crate time;

mod args;
mod async;
mod cursive_views;
mod common;
mod error;
mod view;


fn run() -> error::Result<()> {
    args::process_args()?;
    let mut siv = view::create_cursive_session()?;

    for i in 0.. {
        if i % 2 == 0 {
            async::install_async_handlers(&mut siv);
        }

        siv.step();
    }

    Ok(())
}

quick_main!(run);
