use webservice::{router::init_router, state::AppState};

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::Postgres] conn_string: String) -> shuttle_axum::ShuttleAxum {
    let state = AppState::new(conn_string)
        .await
        .expect("to connect to the database");

    let router = init_router(state);

    Ok(router.into())
}
