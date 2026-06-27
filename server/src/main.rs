mod db;
mod error;
mod models;
mod mqtt;
mod routes;
mod state;
mod ws;

use std::net::SocketAddr;

use axum::{routing::get, routing::post, Router};
use dotenvy::dotenv;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

use crate::state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load .env
    dotenv().ok();

    // Tracing
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "esp32_backend=debug,tower_http=info".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Shared application state (DB pool + WebSocket broadcast)
    let state = AppState::new().await?;

    // Spawn MQTT subscriber → broadcasts data to WS clients and persists to DB
    mqtt::spawn_mqtt_task(state.clone());

    // CORS – allow the frontend dev server and any local origin
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Router
    let app = Router::new()
        // Sensor REST endpoints
        .route("/api/sensors",         get(routes::sensors::get_latest))
        .route("/api/sensors/history", get(routes::sensors::get_history))
        // Control endpoint (publish MQTT command)
        .route("/api/control",         post(routes::control::send_command))
        // Device status
        .route("/api/status",          get(routes::status::get_status))
        // WebSocket endpoint for live data push
        .route("/api/ws",              get(ws::ws_handler))
        .with_state(state)
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".into());
    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".into())
        .parse()?;
    let addr: SocketAddr = format!("{host}:{port}").parse()?;

    info!("🚀 Server listening on http://{addr}");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
