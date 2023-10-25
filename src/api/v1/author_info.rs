
use std::sync::Arc;
use std::collections::HashMap;

use hyper::StatusCode;
use axum::{
    extract::{State, Query},
    response::IntoResponse,
    Json,
};

use crate::state::AppState;
use crate::error_reporting::{ErrorReporting, result_into_response};

use crate::api::v1::types::AuthorInfo;


pub async fn api_author_info(
    State(state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    result_into_response(async move {
        let author_uuid = params.get("uuid").ok_or_else(|| anyhow::anyhow!("Missing uuid parameter"))?;
        
        let mut db_connection = state.db_connection.lock().unwrap();
        let trx = db_connection.transaction()?;

        let (name, created_date, description_text) = if let Ok(values) = trx.query_row(
            "SELECT name, registered_date, description_text FROM author WHERE is_deleted = 0 AND uuid = ?",
            [&author_uuid],
            |row| {
                let name: String = row.get(0)?;
                let created_date: u64 = row.get(1)?;
                let description_text: String = row.get(2)?;
                Ok((name, created_date, description_text))
            }
        ) {
            values
        } else {
            return Ok((
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({
                    "status": "not found",
                })),
            ).into_response());
        };

        let author = AuthorInfo::new(author_uuid, &name, created_date, &description_text);
        Ok(Json(author).into_response())
    }, ErrorReporting::Json).await
}
