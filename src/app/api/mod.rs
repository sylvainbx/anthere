use axum::{Router, routing::{get, post}};

pub fn router() -> Router<()> {
    Router::new()
        .route("/api/xxx", get(|| async { todo!() }))
        .route("/api/yyy", post(|| async { todo!() }))
}