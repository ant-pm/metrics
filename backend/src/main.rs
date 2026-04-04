mod db;
mod routes;

use std::sync::Arc;

use axum::{Router, routing::get};
use tokio_postgres::NoTls;
use tower_http::cors::CorsLayer;

pub struct AppState {
    pub db: tokio_postgres::Client,
}

#[tokio::main]
async fn main() {
    let db_host = std::env::var("QUESTDB_HOST").unwrap_or_else(|_| "localhost".into());
    let db_port = std::env::var("QUESTDB_PG_PORT").unwrap_or_else(|_| "8812".into());

    let conn_str = format!(
        "host={db_host} port={db_port} user=admin password=quest dbname=qdb"
    );

    let (client, connection) = tokio_postgres::connect(&conn_str, NoTls)
        .await
        .expect("failed to connect to questdb");

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("db connection error: {e}");
        }
    });

    let state = Arc::new(AppState { db: client });

    let app = Router::new()
        .route("/api/metrics", get(routes::get_metrics))
        .route("/api/metrics/stream", get(routes::stream_metrics))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
