use std::path::PathBuf;

use app_dirs::{AppDataType, AppInfo, get_app_dir};
use clap::{Arg, App};

use error;


const APP_INFO: AppInfo = AppInfo { name: crate_name!(), author: crate_authors!() };


pub fn process_args() -> error::Result<()> {
    let matches = App::new("Telegram TUI")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(Arg::with_name("config")
            .help("Config file to use")
            .long("config")
            .takes_value(true))
        .arg(Arg::with_name("phone number")
            .help("Sets the phone number used to grant access permissions")
            .long("phone-number")
            .takes_value(true))
        .get_matches();

    let config = matches.value_of("config")
        .map(PathBuf::from)
        .unwrap_or(get_app_dir(AppDataType::UserConfig, &APP_INFO, "config.toml")?);

    Ok(())
}
