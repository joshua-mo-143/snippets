use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use database::{Database, NewSnippet};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct SnippetQuery {
    pub page: Option<i32>,
}

pub async fn list_snippets(
    State(db): State<Database>,
    Query(SnippetQuery { page }): Query<SnippetQuery>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let page = page.unwrap_or_else(|| 0);
    match db.retrieve_snippets_paginated(page).await {
        Ok(snippets) => Ok(Json(snippets)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn list_snippet_by_id(
    State(db): State<Database>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    match db.retrieve_snippet_by_id(id).await {
        Ok(snippet) => Ok(Json(snippet)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn create_snippet(
    State(db): State<Database>,
    Json(snippet): Json<NewSnippet>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    if let Err(e) = db.insert_snippet(snippet).await {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
    };

    Ok(StatusCode::CREATED)
}
