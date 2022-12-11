use std::fs::read_to_string;

use actix_toolbox::logging::setup_logging;
use clap::Parser;
use log::error;
use rorm::{Database, DatabaseConfiguration, DatabaseDriver};

use crate::models::Config;
use crate::server::start_server;

mod handler;
mod models;
mod server;

#[derive(Parser)]
struct Cli {
    #[clap(default_value_t = String::from("config.toml"))]
    #[clap(long = "config-path")]
    #[clap(help = "Specify an alternative path to the configuration file.")]
    config_path: String,
}

#[rorm::rorm_main]
#[tokio::main]
async fn main() -> Result<(), String> {
    let cli = Cli::parse();

    let config_string = read_to_string(&cli.config_path)
        .map_err(|e| format!("Could not read {}: {}", &cli.config_path, e))?;
    let config: Config = toml::from_str(&config_string)
        .map_err(|e| format!("Could not parse {}: {}", &cli.config_path, e))?;

    setup_logging(&config.logging)?;

    let db = Database::connect(DatabaseConfiguration {
        driver: DatabaseDriver::SQLite {
            filename: config.database.filename.clone(),
        },
        max_connections: 5,
        min_connections: 1,
        disable_logging: None,
        slow_statement_log_level: None,
        statement_log_level: None,
    })
    .await
    .map_err(|e| format!("Error initializing database: {e}"))?;

    if let Err(err) = start_server(config, db).await {
        error!("{err}");
    }

    Ok(())
}
