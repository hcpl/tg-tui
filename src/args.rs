use std::collections::HashMap;
use std::path::{Path, PathBuf};

use app_dirs::{AppDataType, AppInfo, get_app_dir};
use config::{Config, File};
use failure;
use structopt::StructOpt;

use app_config::AppConfig;


const APP_INFO: AppInfo = AppInfo { name: crate_name!(), author: crate_authors!() };
const DEFAULT_CONFIG_FILENAME: &'static str = "config.toml";
const DEFAULT_BINDINGS_FILENAME: &'static str = "bindings.toml";


#[derive(Debug, StructOpt)]
struct ArgsConfig {
    #[structopt(long = "config", help = "Config file")]
    config_file: Option<String>,

    #[structopt(long = "bindings", help = "Key bindings file")]
    bindings_file: Option<String>,

    #[structopt(long = "phone-number", help = "The phone number used to grant access permissions")]
    phone_number: Option<String>,
}


/// Process arguments passed to this process and generate an application config.
pub fn process_args() -> Result<AppConfig, failure::Error> {
    let args_config = ArgsConfig::from_args();

    let maybe_config_path = get_maybe_config_path(DEFAULT_CONFIG_FILENAME, &args_config.config_file)?;
    let mut config = process_config_file(maybe_config_path)?;
    process_arg("phone-number", &args_config.phone_number, &mut config)?;

    let maybe_bindings_path = get_maybe_config_path(DEFAULT_BINDINGS_FILENAME, &args_config.bindings_file)?;
    let bindings: HashMap<String, HashMap<String, String>> = process_config_file(maybe_bindings_path)?
        // We're going to discard non key-value configs because:
        // - They are incorrect (at least for now, in future this restriction may be lifted, but
        //   this is unlikely - those are bindings after all);
        // - We can't thoroughly inspect the cache field of Config type, because the
        //   `config::value` module is not public and `config::value::ValueKind` type is not
        //   publicly reexported, even though `ValueKind` itself declared public.
        .try_into()
        .unwrap_or(HashMap::new());
    config.set("bindings", bindings)?;

    config.try_into().map_err(Into::into)
}

fn get_maybe_config_path(default_filename: &str,
                         arg_filename: &Option<String>)
                        -> Result<Option<PathBuf>, failure::Error> {
    let default_config_path = get_app_dir(AppDataType::UserConfig, &APP_INFO, default_filename)?;

    let maybe_default_config_path = if default_config_path.exists() {
        Some(default_config_path)
    } else {
        None
    };

    let maybe_config_path = arg_filename.as_ref()
        .map(PathBuf::from)
        .or(maybe_default_config_path);

    Ok(maybe_config_path)
}

fn process_config_file<P: AsRef<Path>>(config_path: Option<P>) -> Result<Config, failure::Error> {
    let mut config = Config::new();

    if let Some(path) = config_path {
        config.merge(File::with_name(path.as_ref().to_str().unwrap()))?;
    }

    Ok(config)
}

fn process_arg(arg_name: &str, arg_value: &Option<String>, config: &mut Config) -> Result<(), failure::Error> {
    if let Some(ref value) = *arg_value {
        config.set(arg_name, value.as_str())?;
    }

    Ok(())
}
