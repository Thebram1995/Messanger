use axum::{
    http::{HeaderMap, Method, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use reqwest::Client;
use serde_json::Value;

use crate::{
    config::app_state::AppState,
    proxy::service_registry::{Microservice, ServiceRegistry},
};

pub struct ProxyEngine;

impl ProxyEngine {
    pub async fn forward(
        state: &AppState,
        method: Method,
        headers: HeaderMap,
        microservice: Microservice,
        target_path: &str,
        body: Option<Value>,
    ) -> Response {
        let url = ServiceRegistry::resolve(state, microservice, target_path);

        let mut request = state.client.request(method, url);

        // Authorization
        if let Some(auth) = headers.get("Authorization") {
            if let Ok(value) = auth.to_str() {
                request = request.header("Authorization", value);
            }
        }

        // Body
        if let Some(json) = body {
            request = request.json(&json);
        }

        match request.send().await {
            Ok(resp) => {
                let status = StatusCode::from_u16(resp.status().as_u16())
                    .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

                match resp.text().await {
                    Ok(body) => {
                        match serde_json::from_str::<Value>(&body) {
                            Ok(json) => (status, Json(json)).into_response(),

                            Err(_) => (status, body).into_response(),
                        }
                    }

                    Err(error) => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!(
                            "Error leyendo respuesta del microservicio: {error}"
                        ),
                    )
                        .into_response(),
                }
            }

            Err(err) => (
                StatusCode::BAD_GATEWAY,
                format!("Error conectando con microservicio: {}", err),
            )
                .into_response(),
        }
    }
}