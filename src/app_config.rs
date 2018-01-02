use bindings::Bindings;


const DEFAULT_DATE_TIME_FORMAT: &'static str = "%H:%M:%D";


/// Config used for the lifetime of an application process.
#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct AppConfig {
    pub phone_number: Option<String>,
    pub bindings: Bindings,
    pub date_time_format: String,
}

impl Default for AppConfig {
    fn default() -> AppConfig {
        AppConfig {
            phone_number: None,
            bindings: Bindings::default(),
            date_time_format: DEFAULT_DATE_TIME_FORMAT.to_owned(),
        }
    }
}
