use bindings::Bindings;


/// Config used for the lifetime of an application process.
#[derive(Debug, Deserialize)]
pub struct AppConfig {
    #[serde(default)]
    pub phone_number: Option<String>,
    #[serde(default)]
    pub bindings: Bindings,
}
