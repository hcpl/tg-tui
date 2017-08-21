extern crate app_dirs;
extern crate chrono;
#[macro_use]
extern crate clap;
extern crate config;
#[macro_use]
extern crate cursive;
#[macro_use]
extern crate error_chain;
extern crate pom;
extern crate rand;
extern crate textwrap;

mod args;
mod async;
mod cursive_views;
mod commands;
mod common;
mod error;
mod mode;
mod utils;
mod view;


fn run() -> error::Result<()> {
    let config = args::process_args()?;
    let mut siv = view::create_cursive_session(&config)?;

    for i in 0.. {
        if i % 2 == 0 {
            async::install_async_handlers(&mut siv);
        }

        siv.step();
    }

    Ok(())
}

quick_main!(run);
