use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

use database::{Database, NewSnippet};

use search::{FetchParams, Tantivy};
use serde::Deserialize;

use crate::state::AppState;

#[derive(Deserialize)]
pub struct SnippetQuery {
    pub page: Option<i64>,
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
    State(mut state): State<AppState>,
    Json(snippet): Json<NewSnippet>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let doc = match state.db.insert_snippet(snippet).await {
        Ok(doc) => doc,
        Err(err) => return Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string())),
    };

    if let Err(err) = state.search.create_doc(doc) {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, err.to_string()));
    }

    Ok(StatusCode::CREATED)
}

#[derive(Clone, Deserialize)]
pub struct SnippetSearchQuery {
    query: String,
}

pub async fn search_snippets(
    State(search): State<Tantivy>,
    Json(query): Json<SnippetSearchQuery>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let SnippetSearchQuery { query } = query;

    let params = FetchParams::builder().query(query).results_num(5).build();

    let docs = match search.fetch_docs(params) {
        Ok(docs) => docs,
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    };

    Ok(Json(docs))
}
