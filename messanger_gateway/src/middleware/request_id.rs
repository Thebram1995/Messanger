use axum::{
    body::Body,
    http::{HeaderValue, Request},
    middleware::Next,
    response::Response,
};
use uuid::Uuid;

pub async fn request_id_middleware(mut request: Request<Body>, next: Next) -> Response {
    let request_id = Uuid::new_v4().to_string();

    if let Ok(value) = HeaderValue::from_str(&request_id) {
        request.headers_mut().insert("x-request-id", value.clone());

        let mut response = next.run(request).await;

        response.headers_mut().insert("x-request-id", value);

        return response;
    }

    next.run(request).await
}
