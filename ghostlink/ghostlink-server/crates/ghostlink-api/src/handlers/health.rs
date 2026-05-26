use axum::{extract::State, http::StatusCode, Json};
use serde_json::{json, Value};

use crate::AppState;

/// GET /health — liveness probe (always OK if server is running)
pub async fn health() -> StatusCode {
    StatusCode::OK
}

/// GET /health/ready — readiness probe (checks ScyllaDB + Redis connectivity)
pub async fn ready(
    State(state): State<AppState>,
) -> (StatusCode, Json<Value>) {
    let mut healthy = true;
    let mut checks = Vec::new();

    // Check ScyllaDB
    match state.db.scylla.query("SELECT release_version FROM system.local", &[]).await {
        Ok(_) => checks.push(json!({"name": "scylladb", "status": "ok"})),
        Err(e) => {
            checks.push(json!({"name": "scylladb", "status": "error", "message": e.to_string()}));
            healthy = false;
        }
    }

    // Check Redis
    match state.db.redis.get().await {
        Ok(mut conn) => {
            let _: String = redis::cmd("PING").query_async(&mut conn).await.unwrap_or_default();
            checks.push(json!({"name": "redis", "status": "ok"}));
        }
        Err(e) => {
            checks.push(json!({"name": "redis", "status": "error", "message": e.to_string()}));
            healthy = false;
        }
    }

    let status_code = if healthy { StatusCode::OK } else { StatusCode::SERVICE_UNAVAILABLE };
    let status_str = if healthy { "ready" } else { "unhealthy" };

    (status_code, Json(json!({
        "status": status_str,
        "checks": checks
    })))
}
