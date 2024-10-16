use axum::extract::FromRef;
use database::Database;

#[derive(Clone)]
pub struct AppState {
    pub db: Database,
}

impl AppState {
    pub async fn new(conn_string: String) -> Self {
        let db = Database::new(conn_string)
            .await
            .expect("to connect to the database");

        db.execute_migrations().await.unwrap();

        Self { db }
    }
}

impl FromRef<AppState> for Database {
    fn from_ref(input: &AppState) -> Self {
        input.db.clone()
    }
}
