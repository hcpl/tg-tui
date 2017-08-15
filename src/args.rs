use std::path::{Path, PathBuf};

use app_dirs::{AppDataType, AppInfo, get_app_dir};
use clap::{Arg, App};
use config::{Config, File};

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

    let default_config_path = get_app_dir(AppDataType::UserConfig, &APP_INFO, "config.toml")?;
    let maybe_default_config_path = if default_config_path.exists() {
        Some(default_config_path)
    } else {
        None
    };

    // Defaults to config.toml in the app dir if present
    let maybe_config_path = matches.value_of("config")
        .map(PathBuf::from)
        .or(maybe_default_config_path);

    let config = process_config_file(maybe_config_path)?;

    Ok(())
}

fn process_config_file<P: AsRef<Path>>(config_path: Option<P>) -> error::Result<Config> {
    let mut config = Config::new();

    if let Some(path) = config_path {
        config
            .merge(File::with_name(path.as_ref().to_str().unwrap()))?;
    }

    Ok(config)
}
