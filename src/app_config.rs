use bindings::Bindings;


#[derive(Debug, Deserialize)]
pub struct AppConfig {
    #[serde(default)]
    pub phone_number: Option<String>,
    #[serde(default)]
    pub bindings: Bindings,
}
