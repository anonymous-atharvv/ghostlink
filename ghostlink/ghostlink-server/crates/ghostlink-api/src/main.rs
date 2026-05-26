use ghostlink_api::{config::AppConfig, router::create_router, AppState};
use ghostlink_db::{connection::DatabaseConfig, migrations, Database};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load .env file (dev only)
    let _ = dotenvy::dotenv();

    // Initialize tracing (privacy-safe: NO PII in logs)
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(tracing_subscriber::fmt::layer().json())
        .init();

    tracing::info!("🔒 GhostLink server starting...");

    // Load configuration
    let config = AppConfig::from_env();
    tracing::info!(
        host = %config.server_host,
        port = %config.server_port,
        "Configuration loaded"
    );

    // Connect to databases
    let db_config = DatabaseConfig {
        scylla_nodes: config.scylla_nodes.clone(),
        scylla_keyspace: config.scylla_keyspace.clone(),
        scylla_username: config.scylla_username.clone(),
        scylla_password: config.scylla_password.clone(),
        redis_url: config.redis_url.clone(),
    };

    let db = Database::connect(&db_config).await?;

    // Run migrations
    migrations::run_migrations(&db.scylla, &config.scylla_keyspace).await?;

    // Connect to NATS
    let nats_client = match async_nats::connect(&config.nats_url).await {
        Ok(client) => {
            tracing::info!("NATS connected to {}", config.nats_url);
            Some(client)
        }
        Err(e) => {
            tracing::warn!("Failed to connect to NATS at {}: {}. Running in single-pod mode.", config.nats_url, e);
            None
        }
    };

    // Build application state
    let state = AppState::new(config.clone(), db, nats_client.clone());

    // Spin up NATS cross-pod WebSocket bridge in background if NATS is active
    if nats_client.is_some() {
        let hub_clone = state.hub.clone();
        let nats_url = config.nats_url.clone();
        tokio::spawn(async move {
            match ghostlink_ws::nats_bridge::NatsBridge::new(&nats_url, hub_clone).await {
                Ok(bridge) => {
                    if let Err(e) = bridge.start_subscriber().await {
                        tracing::error!(error = %e, "NATS subscriber bridge task failed");
                    }
                }
                Err(e) => {
                    tracing::error!(error = %e, "Failed to initialize NATS bridge");
                }
            }
        });
    }

    // Build router
    let app = create_router(state);

    // Start server
    let addr = format!("{}:{}", config.server_host, config.server_port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!(address = %addr, "🚀 GhostLink server listening");

    axum::serve(listener, app).await?;

    Ok(())
}
