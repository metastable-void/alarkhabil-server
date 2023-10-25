
use std::sync::Arc;
use std::collections::HashMap;

use axum::{
    extract::{State, Query},
    response::IntoResponse,
    Json,
};

use crate::state::AppState;
use crate::error_reporting::{ErrorReporting, result_into_response};

use crate::api::v1::types::ChannelSummary;


pub async fn api_channel_list(
    State(state): State<Arc<AppState>>,
    Query(_params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    result_into_response(async move {
        let mut db_connection = state.db_connection.lock().unwrap();
        let trx = db_connection.transaction()?;

        let mut stmt = trx.prepare(
            "SELECT uuid, handle, name, language_code FROM channel WHERE is_deleted = 0 ORDER BY created_date DESC LIMIT 1000"
        )?;

        let mut rows = stmt.query([])?;
        let mut channels = Vec::new();

        while let Some(row) = rows.next()? {
            let channel_uuid: String = row.get(0)?;
            let handle: String = row.get(1)?;
            let name: String = row.get(2)?;
            let language_code: String = row.get(3)?;
            channels.push(ChannelSummary::new(&channel_uuid, &handle, &name, &language_code));
        }

        Ok(Json(serde_json::json!(channels)))
    }, ErrorReporting::Json).await
}
