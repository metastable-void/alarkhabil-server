
use std::collections::HashMap;
use std::sync::Arc;

use axum::{
    extract::{State, Query},
    response::IntoResponse,
    Json,
};
use hyper::StatusCode;

use crate::state::AppState;
use crate::error_reporting::{ErrorReporting, result_into_response};


pub async fn api_meta_info(
    State(state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    result_into_response(async move {
        let page_name = params.get("page_name").ok_or_else(|| anyhow::anyhow!("Missing page_name parameter"))?;

        let db_connection = state.db_connection.lock().unwrap();

        let (title, updated_date, page_text) = if let Ok(values) = db_connection.query_row(
            "SELECT title, updated_date, page_text FROM meta_page WHERE page_name = ?", 
            [page_name],
            |row| {
                let title: String = row.get(0)?;
                let updated_date: u64 = row.get(1)?;
                let page_text: String = row.get(2)?;

                Ok((title, updated_date, page_text))
            },
        ) {
            values
        } else {
            return Ok((
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({
                    "status": "error",
                    "error": "not found",
                })),
            ).into_response());
        };
        
        Ok(Json(serde_json::json!({
            "page_name": page_name,
            "updated_date": updated_date,
            "title": title,
            "text": &page_text,
        })).into_response())
    }, ErrorReporting::Json).await
}
