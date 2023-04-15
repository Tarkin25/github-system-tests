use std::net::SocketAddr;

use anyhow::Context;
use axum::{routing::get, Router};
use configuration::{init_logging, Configuration};

mod configuration;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let config = Configuration::new()?;
    init_logging(&config);

    tracing::debug!("{config:?}");

    let app = Router::new().route("/", get(|| async { "it works" }));

    let address = SocketAddr::from(([0, 0, 0, 0], config.server_port));

    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .context("server error")
}
