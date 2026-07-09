use axum::Json;
use serde_json::{json, Value};

#[utoipa::path(
    get,
    path = "/health",
    tag = "Health",
    responses(
        (status = 200, description = "Health check")
    )
)]

pub async fn health() -> Json<Value> {
    Json(json!({
        "status": "UP",
        "service": "messanger"
    }))
}