use std::net::SocketAddr;

use anyhow::Context;
use axum::{extract::State, response::IntoResponse, routing::get, Router};
use configuration::{init_logging, Configuration};
use sqlx::PgPool;

mod configuration;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let config = Configuration::new()?;
    init_logging(&config);
    tracing::debug!("{config:?}");

    let pool = config.create_pool().await?;

    let app = Router::new().route("/", get(index)).with_state(pool);

    let address = SocketAddr::from(([0, 0, 0, 0], config.server_port));

    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .context("server error")
}

async fn index(State(pool): State<PgPool>) -> impl IntoResponse {
    let (hello, world): (String, String) = sqlx::query_as("select 'hello', ' world'")
        .fetch_one(&pool)
        .await
        .unwrap();

    hello + &world
}
