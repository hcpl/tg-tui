extern crate app_dirs;
extern crate chrono;
#[macro_use]
extern crate clap;
extern crate config;
#[macro_use]
extern crate cursive;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate maplit;
extern crate pom;
extern crate rand;
#[macro_use]
extern crate serde_derive;
extern crate textwrap;


mod action;
mod app_config;
mod args;
mod async;
mod bindings;
mod cursive_views;
mod commands;
mod error;
mod mode;
mod utils;
mod view;


fn run() -> error::Result<()> {
    let mut app_config = args::process_args()?;
    let mut siv = view::create_cursive_session(&mut app_config)?;

    for i in 0.. {
        if i % 2 == 0 {
            async::install_async_handlers(&mut siv);
        }

        siv.step();
    }

    Ok(())
}

quick_main!(run);
