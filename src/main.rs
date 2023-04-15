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

    let (tx, rx) = tokio::sync::oneshot::channel();

    let server = axum::Server::bind(&address)
        .serve(app.into_make_service())
        .with_graceful_shutdown(async {
            rx.await.ok();
        });

    let server_task = tokio::spawn(server);

    tokio::signal::ctrl_c()
        .await
        .context("failed to listen for ctrl+c")?;

    tx.send(()).expect("failed to send shutdown signal");

    server_task
        .await
        .context("failed to join server task")?
        .context("server error")
}
