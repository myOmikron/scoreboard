use std::io;

use actix_web::web::{Data, JsonConfig, PayloadConfig};
use actix_web::{App, HttpServer};
use rorm::Database;

use crate::models::Config;

pub(crate) async fn start_server(config: Config, db: Database) -> Result<(), io::Error> {
    HttpServer::new(move || {
        App::new()
            .app_data(JsonConfig::default())
            .app_data(PayloadConfig::default())
            .app_data(Data::new(db.clone()))
    })
    .bind((
        config.server.listen_address.as_str(),
        config.server.listen_port,
    ))?
    .run()
    .await
}
