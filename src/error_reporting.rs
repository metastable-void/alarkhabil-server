
use std::future::Future;

use serde::{Serialize, Deserialize};

use hyper::StatusCode;

use axum::{
    http::header,
    response::IntoResponse,
    Json,
};


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ErrorReporting {
    Html,
    Json,
}

async fn handle_anyhow_error(err: anyhow::Error, reporting: ErrorReporting) -> impl IntoResponse {
    log::error!("Error: {}", err);

    if ErrorReporting::Json == reporting {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "status": "error",
                "message": err.to_string(),
            })),
        ).into_response();
    }

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        [(header::CONTENT_TYPE, "text/html")],
        "<!doctype html><html><head><title>Internal Server Error</title></head><body><h1>Internal Server Error</h1></body></html>",
    ).into_response()
}

pub async fn result_into_response<T: IntoResponse, Fut: Future<Output = anyhow::Result<T>>>(
    result: Fut,
    error_reporting: ErrorReporting,
) -> impl IntoResponse {
    match result.await {
        Ok(response) => {
            response.into_response()
        },
        Err(e) => {
            handle_anyhow_error(e, error_reporting).await.into_response()
        }
    }
}
