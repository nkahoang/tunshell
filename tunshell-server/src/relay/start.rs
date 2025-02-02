use super::{config::Config, server::Server};
use crate::db;
use anyhow::Result;
use log::*;

pub async fn start() -> Result<()> {
    let config = Config::from_env()?;

    let sessions = db::SessionStore::new(db::connect().await?);

    info!(
        "starting relay server on ports (tls: {}, ws: {})",
        config.tls_port, config.ws_port
    );

    Server::new(config, sessions).start(None).await
}
