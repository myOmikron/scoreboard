use actix_toolbox::logging::LoggingConfig;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct Database {
    pub(crate) filename: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct Server {
    pub(crate) listen_address: String,
    pub(crate) listen_port: u16,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct Config {
    pub(crate) database: Database,
    pub(crate) server: Server,
    pub(crate) logging: LoggingConfig,
}
