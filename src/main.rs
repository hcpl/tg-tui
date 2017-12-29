extern crate app_dirs;
extern crate chrono;
#[macro_use]
extern crate clap;
extern crate config;
#[macro_use]
extern crate cursive;
extern crate env_logger;
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
extern crate structopt;
#[macro_use]
extern crate structopt_derive;
extern crate textwrap;


#[cfg(not(any(
    all(feature = "blt-backend",
        not(feature = "ncurses-backend"),
        not(feature = "pancurses-backend"),
        not(feature = "termion-backend")),

    all(not(feature = "blt-backend"),
        feature = "ncurses-backend",
        not(feature = "pancurses-backend"),
        not(feature = "termion-backend")),

    all(not(feature = "blt-backend"),
        not(feature = "ncurses-backend"),
        feature = "pancurses-backend",
        not(feature = "termion-backend")),

    all(not(feature = "blt-backend"),
        not(feature = "ncurses-backend"),
        not(feature = "pancurses-backend"),
        feature = "termion-backend"),
)))]
compile_error!("Exactly one backend must be used with this crate. \
    Please specify either of:
        `features = [\"blt-backend\"]`
        `features = [\"ncurses-backend\"]`
        `features = [\"pancurses-backend\"]`
        `features = [\"termion-backend\"]`");


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
    env_logger::init();

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
