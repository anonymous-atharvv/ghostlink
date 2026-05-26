use axum::{extract::Request, middleware::Next, response::Response};
use uuid::Uuid;

/// Request ID middleware.
/// Generates a unique ID for each request for tracing.
/// NEVER logs user-identifying information.
pub async fn request_id_middleware(
    mut req: Request,
    next: Next,
) -> Response {
    let request_id = Uuid::new_v4().to_string();
    req.extensions_mut().insert(RequestId(request_id.clone()));

    let start = std::time::Instant::now();
    let method = req.method().clone();
    let path = req.uri().path().to_owned();

    let response = next.run(req).await;

    let latency = start.elapsed().as_millis();

    // Privacy-safe logging: ONLY these fields, NEVER IP/username/body
    tracing::info!(
        request_id = %request_id,
        method = %method,
        path = %path,
        status = %response.status().as_u16(),
        latency_ms = %latency,
    );

    response
}

/// Request ID extension type
#[derive(Debug, Clone)]
pub struct RequestId(pub String);
