use std::collections::HashMap;
use std::path::{Path, PathBuf};

use app_dirs::{AppDataType, AppInfo, get_app_dir};
use clap::{App, Arg, ArgMatches};
use config::{Config, File};

use app_config::AppConfig;
use error;


const APP_INFO: AppInfo = AppInfo { name: crate_name!(), author: crate_authors!() };
const DEFAULT_CONFIG_FILENAME: &'static str = "config.toml";
const DEFAULT_BINDINGS_FILENAME: &'static str = "bindings.toml";


pub fn process_args() -> error::Result<AppConfig> {
    let matches = App::new("Telegram TUI")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(Arg::with_name("config")
            .help("Config file to use")
            .long("config")
            .takes_value(true))
        .arg(Arg::with_name("bindings")
            .help("Bindings file to use")
            .long("bindings")
            .takes_value(true))
        .arg(Arg::with_name("phone-number")
            .help("The phone number used to grant access permissions")
            .long("phone-number")
            .takes_value(true))
        .get_matches();

    let maybe_config_path = get_maybe_config_path(DEFAULT_CONFIG_FILENAME, &matches, "config")?;
    let mut config = process_config_file(maybe_config_path)?;
    process_arg(&matches, "phone-number", &mut config)?;

    let maybe_bindings_path = get_maybe_config_path(DEFAULT_BINDINGS_FILENAME, &matches, "bindings")?;
    let bindings: HashMap<String, String> = process_config_file(maybe_bindings_path)?
        // We're going to discard non key-value configs because:
        // - They are incorrect (at least for now, in future this restriction may be lifted, but
        //   this is unlikely - those are bindings after all);
        // - We can't thoroughly inspect the cache field of Config type, because the
        //   `config::value` module is not public and `config::value::ValueKind` type is not
        //   publicly reexported, even though `ValueKind` itself declared public.
        .try_into()
        .unwrap_or(HashMap::new());
    config.set("bindings", bindings)?;
    println!("{:#?}", config);

    let x = config.try_into();
    println!("{:#?}", x);

    x.map_err(Into::into)
}

fn get_maybe_config_path(default_filename: &str, matches: &ArgMatches, arg_name: &str) -> error::Result<Option<PathBuf>> {
    let default_config_path = get_app_dir(AppDataType::UserConfig, &APP_INFO, default_filename)?;

    let maybe_default_config_path = if default_config_path.exists() {
        Some(default_config_path)
    } else {
        None
    };

    let maybe_config_path = matches.value_of(arg_name)
        .map(PathBuf::from)
        .or(maybe_default_config_path);

    Ok(maybe_config_path)
}

fn process_config_file<P: AsRef<Path>>(config_path: Option<P>) -> error::Result<Config> {
    let mut config = Config::new();

    if let Some(path) = config_path {
        config.merge(File::with_name(path.as_ref().to_str().unwrap()))?;
    }

    Ok(config)
}

fn process_arg(matches: &ArgMatches, arg_name: &str, config: &mut Config) -> error::Result<()> {
    if let Some(value) = matches.value_of(arg_name) {
        config.set(arg_name, value)?;
    }

    Ok(())
}
