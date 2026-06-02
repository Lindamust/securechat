#[allow(unused)]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use std::{path::Path, sync::Arc};

use anyhow::Ok;
use axum::{Router, routing::post};
use infra::database::PgDatabase;
use sqlx::postgres::PgPoolOptions;
use tower_http::limit::RequestBodyLimitLayer;

mod routes;
mod steps;
mod telemetry;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::from_filename(Path::new(env!("CARGO_MANIFEST_DIR")).join("../infra/.env")).ok();

    let _otel_guard = telemetry::init("securechat-server").await?;

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    let executor = Arc::new(PgDatabase { pool });

    let app = Router::new()
        .route("/api/register", post(routes::register_pipeline()))
        .route("/auth/challenge", post(routes::auth_challenge_pipeline()))
        .layer(RequestBodyLimitLayer::new(64 * 1_024))
        .layer(telemetry::http_trace_layer())
        .with_state(executor);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:5000").await?;
    tracing::info!("Listening on {}", listener.local_addr()?);
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to listen for ctrl-c");
    tracing::info!("shutdown signal received");
}
