
use std::sync::Arc;
use std::collections::HashMap;

use axum::{
    extract::{State, Query},
    response::IntoResponse,
    Json,
};

use crate::state::AppState;
use crate::error_reporting::{ErrorReporting, result_into_response};

use crate::api::v1::types::AuthorSummary;


pub async fn api_author_list(
    State(state): State<Arc<AppState>>,
    Query(_params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    result_into_response(async move {
        let mut db_connection = state.db_connection.lock().unwrap();
        let trx = db_connection.transaction()?;

        let mut stmt = trx.prepare(
            "SELECT uuid, name FROM author WHERE is_deleted = 0 ORDER BY registered_date DESC LIMIT 1000"
        )?;

        let mut rows = stmt.query([])?;
        let mut authors = Vec::new();

        while let Some(row) = rows.next()? {
            let author_uuid: String = row.get(0)?;
            let name: String = row.get(1)?;
            authors.push(AuthorSummary::new(&author_uuid, &name));
        }

        Ok(Json(serde_json::json!(authors)))
    }, ErrorReporting::Json).await
}
