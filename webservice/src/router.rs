use axum::{routing::get, Router};
use tower_http::services::{ServeDir, ServeFile};

use crate::{
    endpoints::snippets::{create_snippet, list_snippet_by_id, list_snippets},
    state::AppState,
};

pub fn init_router(state: AppState) -> Router {
    Router::new()
        .route("/api/snippets/:id", get(list_snippet_by_id))
        .route("/api/snippets", get(list_snippets).post(create_snippet))
        .nest_service(
            "/",
            ServeDir::new("frontend-dist").fallback(ServeFile::new("frontend-dist/index.html")),
        )
        .with_state(state)
}
