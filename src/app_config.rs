use bindings::Bindings;


#[derive(Deserialize)]
pub struct AppConfig {
    pub phone_number: Option<String>,
    pub bindings: Bindings,
}
