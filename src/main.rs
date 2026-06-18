mod auth;
mod config;
mod engines;
mod error;
mod integrations;
mod limits;
mod mcp;
mod models;
mod state;
mod telemetry;
mod tools;
mod util;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Load config (reads .env and env vars)
    let config = config::Config::load().unwrap_or_else(|e| {
        eprintln!("config error: {e}; using defaults");
        config::Config::default()
    });

    // Initialize tracing
    telemetry::init_tracing(&config.log_level);

    tracing::info!(
        version = env!("CARGO_PKG_VERSION"),
        host = %config.host,
        port = config.port,
        "ai-scraping-defense-mcp starting"
    );

    // Build application state
    let app_state = state::AppState::new(config);

    // Run the MCP server (blocks until shutdown)
    mcp::server::run_server(app_state).await
}
