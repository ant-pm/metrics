use std::sync::Arc;
use std::time::Duration;

use axum::extract::State;
use axum::response::sse::{Event, KeepAlive, Sse};
use axum::response::{IntoResponse, Json};
use futures::stream::Stream;
use serde::Serialize;

use crate::AppState;
use crate::db::{query_history, query_latest_metrics};

#[derive(Serialize)]
struct FullResponse {
    #[serde(flatten)]
    metrics: crate::db::MetricsResponse,
    history: crate::db::HistoryResponse,
}

async fn fetch_all(state: &AppState) -> Result<FullResponse, Box<dyn std::error::Error + Send + Sync>> {
    let client = state.pool.get().await?;
    let (metrics, history) = tokio::try_join!(
        query_latest_metrics(&client),
        query_history(&client),
    )?;
    Ok(FullResponse { metrics, history })
}

pub async fn get_metrics(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    match fetch_all(&state).await {
        Ok(data) => Json(data).into_response(),
        Err(e) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": format!("query error: {e}") })),
        )
            .into_response(),
    }
}

pub async fn stream_metrics(
    State(state): State<Arc<AppState>>,
) -> Sse<impl Stream<Item = Result<Event, axum::Error>>> {
    let stream = async_stream::stream! {
        loop {
            tokio::time::sleep(Duration::from_secs(10)).await;
            match fetch_all(&state).await {
                Ok(data) => {
                    let json = serde_json::to_string(&data).unwrap();
                    yield Ok(Event::default().data(json));
                }
                Err(e) => {
                    eprintln!("SSE query error: {e}");
                }
            }
        }
    };

    Sse::new(stream).keep_alive(KeepAlive::default())
}
