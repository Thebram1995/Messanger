#[macro_export]
macro_rules! proxy_route {
    (
        POST,
        $function_name:ident,
        $microservice:expr,
        $target_path:expr
    ) => {
        pub async fn $function_name(
            axum::extract::State(state): axum::extract::State<crate::config::app_state::AppState>,
            headers: axum::http::HeaderMap,
            axum::Json(payload): axum::Json<serde_json::Value>,
        ) -> axum::response::Response {
            crate::proxy::proxy_engine::ProxyEngine::forward(
                &state,
                axum::http::Method::POST,
                headers,
                $microservice,
                $target_path,
                Some(payload),
            )
            .await
        }
    };

    (
        GET,
        $function_name:ident,
        $microservice:expr,
        $target_path:expr
    ) => {
        pub async fn $function_name(
            axum::extract::State(state): axum::extract::State<crate::config::app_state::AppState>,
            headers: axum::http::HeaderMap,
        ) -> axum::response::Response {
            crate::proxy::proxy_engine::ProxyEngine::forward(
                &state,
                axum::http::Method::GET,
                headers,
                $microservice,
                $target_path,
                None,
            )
            .await
        }
    };
}