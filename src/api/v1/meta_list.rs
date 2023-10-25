
use std::collections::HashMap;
use std::sync::Arc;

use axum::{
    extract::{State, Query},
    response::IntoResponse,
    Json,
};

use crate::state::AppState;
use crate::error_reporting::{ErrorReporting, result_into_response};


pub async fn api_meta_list(
    State(state): State<Arc<AppState>>,
    Query(_params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    result_into_response(async move {
        let db_connection = state.db_connection.lock().unwrap();

        let mut stmt = db_connection.prepare(
            "SELECT page_name, title, updated_date FROM meta_page ORDER BY updated_date DESC LIMIT 1000",
        )?;

        let mut rows = stmt.query([])?;

        let mut pages = Vec::<serde_json::Value>::new();
        while let Some(row) = rows.next()? {
            let page_name: String = row.get(0)?;
            let title: String = row.get(1)?;
            let updated_date: u64 = row.get(2)?;

            pages.push(serde_json::json!({
                "page_name": page_name,
                "updated_date": updated_date,
                "title": title,
            }));
        }
        
        Ok(Json(serde_json::json!(pages)).into_response())
    }, ErrorReporting::Json).await
}
