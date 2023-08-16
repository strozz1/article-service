use config::{Config, FileFormat};
use serde::Deserialize;



#[derive(Clone,Deserialize)]
pub struct AppConfig{
    pub host: String,
    pub port: u16,
    pub db_config: DatabaseConfig
}


#[derive(Clone,Deserialize)]
pub struct DatabaseConfig{
    pub host: String,
    pub port: u16,
    pub db: String,
    pub collection: String
}

///Tries deserilizing the app configuration struct from the file specified in the add_source method and returns a Endpoints struct with endpoints for the api gateway.
/// Panics if error occurred.
pub fn get_app_config() -> AppConfig {
    let config: Config = Config::builder()
        .add_source(config::File::new("app-config", FileFormat::Toml))
        .build()
        .expect("Error loading the app configuration");
    let app_config = config
        .try_deserialize::<AppConfig>()
        .expect("Error deserializing the app configuration.");
    app_config
}


