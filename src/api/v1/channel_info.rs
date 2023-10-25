
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

use crate::api::v1::types::ChannelInfo;


enum QueryType {
    ByUuid(String),
    ByHandle(String),
}

pub async fn api_channel_info(
    State(state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    result_into_response(async move {
        let maybe_channel_uuid = params.get("uuid");
        let maybe_channel_handle = params.get("handle");

        if maybe_channel_handle.is_none() && maybe_channel_uuid.is_none() {
            return Err(anyhow::anyhow!("Missing uuid or handle parameter"));
        }

        if maybe_channel_handle.is_some() && maybe_channel_uuid.is_some() {
            return Err(anyhow::anyhow!("Both uuid and handle parameters are present"));
        }

        let query_type = if let Some(channel_uuid) = maybe_channel_uuid {
            QueryType::ByUuid(channel_uuid.to_owned())
        } else if let Some(channel_handle) = maybe_channel_handle {
            QueryType::ByHandle(channel_handle.to_owned())
        } else {
            unreachable!()
        };
        
        let mut db_connection = state.db_connection.lock().unwrap();
        let trx = db_connection.transaction()?;

        let (name, created_date, description_text, channel_uuid, channel_handle, language_code) = if let Ok(values) = match query_type {
            QueryType::ByUuid(channel_uuid) => trx.query_row(
                "SELECT name, created_date, description_text, handle, language_code FROM channel WHERE is_deleted = 0 AND uuid = ?",
                [channel_uuid.clone()],
                |row| {
                    let name: String = row.get(0)?;
                    let created_date: u64 = row.get(1)?;
                    let description_text: String = row.get(2)?;
                    let channel_handle: String = row.get(3)?;
                    let language_code: String = row.get(4)?;
                    Ok((name, created_date, description_text, channel_uuid, channel_handle, language_code))
                }
            ),
            QueryType::ByHandle(channel_handle) => trx.query_row(
                "SELECT name, created_date, description_text, uuid, language_code FROM channel WHERE is_deleted = 0 AND handle = ?",
                [channel_handle.clone()],
                |row| {
                    let name: String = row.get(0)?;
                    let created_date: u64 = row.get(1)?;
                    let description_text: String = row.get(2)?;
                    let channel_uuid: String = row.get(3)?;
                    let language_code: String = row.get(4)?;
                    Ok((name, created_date, description_text, channel_uuid, channel_handle, language_code))
                }
            ),
        } {
            values
        } else {
            return Ok((
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({
                    "status": "not found",
                })),
            ).into_response());
        };

        let channel = ChannelInfo::new(&channel_uuid, &channel_handle, &name, created_date, &language_code, &description_text);
        Ok(Json(channel).into_response())
    }, ErrorReporting::Json).await
}
