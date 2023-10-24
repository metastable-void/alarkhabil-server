
use std::collections::HashMap;
use std::sync::Arc;

use axum::{
    extract::{State, Query},
    response::IntoResponse,
    Json,
};

use serde::{Serialize, Deserialize};

use crate::state::AppState;
use crate::error_reporting::{ErrorReporting, result_into_response};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MsgMetaUpdate {
    page_name: String,
    title: String,
    text: String,
}

pub async fn api_admin_meta_update(
    State(state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
    Json(msg): Json<MsgMetaUpdate>,
) -> impl IntoResponse {
    result_into_response(async move {
        let admin_token = state.primary_secret.derive_secret("admin_token");
        let token = hex::decode(params.get("token").ok_or_else(|| anyhow::anyhow!("Missing token parameter"))?)?;

        // TODO: prevent time-based attacks
        if admin_token != token {
            return Err(anyhow::anyhow!("Invalid token"));
        }

        let page_name = &msg.page_name;
        let title = &msg.title;
        let text = &msg.text;

        let mut db_connection = state.db_connection.lock().unwrap();
        let trx = db_connection.transaction()?;

        trx.execute("INSERT OR REPLACE INTO meta_page (page_name, title, page_text) VALUES (?, ?, ?)", (page_name, title, text))?;
        trx.commit()?;

        Ok(Json(serde_json::json!({
            "status": "ok",
        })))
    }, ErrorReporting::Json).await
}