use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};
use sqlx::{MySql, Pool};
use std::sync::Arc;

pub mod pasta;

pub struct AppState {
    db: Pool<MySql>,
}

pub async fn run(pool: Pool<MySql>) {
    let shared_state = Arc::new(AppState { db: pool });

    let listener = tokio::net::TcpListener::bind("127.0.0.1:7878")
        .await
        .unwrap();
    axum::serve(listener, api_router(shared_state))
        .await
        .unwrap();
}

pub fn api_router(shared_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/pasta/{slug}", get(pasta::get_pasta))
        .with_state(shared_state)
        .fallback(handler_404)
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}
