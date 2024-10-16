use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Clone)]
pub struct Database {
    pool: PgPool,
}

impl Database {
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
        page: i32,
    ) -> Result<Vec<Snippet>, sqlx::Error> {
        let offset = if page <= 0 { 0 } else { (page - 1) * 10 };
        let snippets: Vec<Snippet> = sqlx::query_as("SELECT * FROM SNIPPETS offset $1 limit 10")
            .bind(offset)
            .fetch_all(&self.pool)
            .await?;

        Ok(snippets)
    }

    pub async fn retrieve_snippet_by_id(&self, slug: i32) -> Result<Snippet, sqlx::Error> {
        let snippet: Snippet = sqlx::query_as("SELECT * FROM snippets WHERE id = $1")
            .bind(slug)
            .fetch_one(&self.pool)
            .await?;

        Ok(snippet)
    }

    pub async fn insert_snippet(&self, snippet: NewSnippet) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO snippets
            (title, description, snippet, language)
            VALUES
            ($1, $2, $3, $4)",
        )
        .bind(snippet.title)
        .bind(snippet.description)
        .bind(snippet.snippet)
        .bind(snippet.language)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}

#[derive(sqlx::FromRow, Serialize)]
pub struct Snippet {
    #[serde(skip_serializing)]
    id: i32,
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
