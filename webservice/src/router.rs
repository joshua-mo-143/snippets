use axum::{
    http::Method,
    routing::{get, post},
    Router,
};
use tower_http::{
    cors::{Any, CorsLayer},
    services::{ServeDir, ServeFile},
};

use crate::{
    endpoints::snippets::{create_snippet, list_snippet_by_id, list_snippets, search_snippets},
    state::AppState,
};

pub fn init_router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_headers(Any)
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any);

    Router::new()
        .route("/api/snippets/:id", get(list_snippet_by_id))
        .route("/api/snippets", get(list_snippets).post(create_snippet))
        .route("/api/search", post(search_snippets))
        .nest_service(
            "/",
            ServeDir::new("dist").fallback(ServeFile::new("dist/index.html")),
        )
        .with_state(state)
        .layer(cors)
}
