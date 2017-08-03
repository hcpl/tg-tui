extern crate app_dirs;
#[macro_use]
extern crate clap;
extern crate cursive;
#[macro_use]
extern crate error_chain;
extern crate time;

mod args;
mod cursive_views;
mod common;
mod error;
mod view;


fn run() -> error::Result<()> {
    args::process_args()?;
    view::create_cursive_session()?.run();

    Ok(())
}

quick_main!(run);
