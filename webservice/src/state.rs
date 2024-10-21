use axum::extract::FromRef;
use database::Database;
use search::Tantivy;

#[derive(Clone)]
pub struct AppState {
    pub db: Database,
    pub search: Tantivy,
}

impl AppState {
    pub async fn new(conn_string: String) -> Result<Self, Box<dyn std::error::Error>> {
        let db = Database::new(conn_string)
            .await
            .expect("to connect to the database");

        db.execute_migrations().await?;
        let mut search = Tantivy::new()?;
        search.seed(&db).await?;

        Ok(Self { db, search })
    }
}

impl FromRef<AppState> for Database {
    fn from_ref(input: &AppState) -> Self {
        input.db.clone()
    }
}

impl FromRef<AppState> for Tantivy {
    fn from_ref(input: &AppState) -> Self {
        input.search.clone()
    }
}
