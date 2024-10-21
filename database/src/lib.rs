use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgPool};

#[derive(Clone)]
pub struct Database {
    pool: PgPool,
}

impl Database {
    /// Creates a new database.
    pub async fn new(conn_string: String) -> Result<Self, sqlx::Error> {
        let pool = PgPool::connect(&conn_string).await?;

        Ok(Self { pool })
    }

    pub async fn execute_migrations(&self) -> Result<(), sqlx::Error> {
        sqlx::migrate!().run(&self.pool).await?;

        Ok(())
    }

    pub async fn retrieve_snippets_paginated(
        &self,
        page: i64,
    ) -> Result<Vec<Snippet>, sqlx::Error> {
        let offset = if page <= 0 { 0 } else { (page - 1) * 10 };
        let snippets: Vec<Snippet> =
            sqlx::query_as("SELECT * FROM SNIPPETS order by id desc offset $1 limit 10")
                .bind(offset)
                .fetch_all(&self.pool)
                .await?;

        Ok(snippets)
    }

    pub async fn retrieve_snippet_by_id(&self, slug: i32) -> Result<Snippet, sqlx::Error> {
        let snippet: Snippet = sqlx::query_as("SELECT * FROM snippets WHERE id = $1 limit 1")
            .bind(slug)
            .fetch_one(&self.pool)
            .await?;

        Ok(snippet)
    }

    pub async fn insert_snippet(
        &self,
        snippet: NewSnippet,
    ) -> Result<SnippetDocument, sqlx::Error> {
        let doc: SnippetDocument = sqlx::query_as(
            "INSERT INTO snippets
            (title, description, snippet, language)
            VALUES
            ($1, $2, $3, $4)
            returning *",
        )
        .bind(snippet.title)
        .bind(snippet.description)
        .bind(snippet.snippet)
        .bind(snippet.language)
        .fetch_one(&self.pool)
        .await?;

        Ok(doc)
    }

    pub async fn retrieve_search_results(&self) -> Result<Vec<SnippetDocument>, sqlx::Error> {
        let search_results =
            sqlx::query_as::<_, SnippetDocument>("SELECT id, title, description FROM snippets")
                .fetch_all(&self.pool)
                .await?;

        Ok(search_results)
    }
}

#[derive(Clone, Deserialize, Serialize, FromRow, Debug)]
pub struct SnippetDocument {
    pub id: i64,
    pub title: String,
    pub description: String,
}

impl<'a> SnippetDocument {
    pub fn id(&self) -> i64 {
        self.id.clone()
    }

    pub fn title(&'a self) -> &'a str {
        &self.title
    }

    pub fn desc(&'a self) -> &'a str {
        &self.description
    }
}
#[derive(sqlx::FromRow, Serialize)]
pub struct Snippet {
    id: i64,
    title: String,
    description: String,
    snippet: String,
    language: String,
    created_at: DateTime<Utc>,
    last_updated_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct NewSnippet {
    title: String,
    description: String,
    snippet: String,
    language: String,
}
