mod db;
mod routes;

use std::sync::Arc;

use axum::{Router, routing::get};
use deadpool_postgres::{Config, Runtime};
use tokio_postgres::NoTls;
use tower_http::cors::CorsLayer;

pub struct AppState {
    pub pool: deadpool_postgres::Pool,
}

#[tokio::main]
async fn main() {
    let db_host = std::env::var("QUESTDB_HOST").unwrap_or_else(|_| "localhost".into());
    let db_port: u16 = std::env::var("QUESTDB_PG_PORT")
        .unwrap_or_else(|_| "8812".into())
        .parse()
        .expect("invalid QUESTDB_PG_PORT");

    let mut cfg = Config::new();
    cfg.host = Some(db_host);
    cfg.port = Some(db_port);
    cfg.dbname = Some("qdb".into());
    cfg.user = Some("admin".into());
    cfg.password = Some("quest".into());

    let pool = cfg
        .create_pool(Some(Runtime::Tokio1), NoTls)
        .expect("failed to create connection pool");

    let state = Arc::new(AppState { pool });

    let app = Router::new()
        .route("/api/metrics", get(routes::get_metrics))
        .route("/api/metrics/stream", get(routes::stream_metrics))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
