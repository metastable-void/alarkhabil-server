
use std::collections::HashMap;
use std::sync::Arc;

use axum::{
    extract::{State, Query},
    response::IntoResponse,
    Json,
};

use crate::state::AppState;
use crate::error_reporting::{ErrorReporting, result_into_response};


pub async fn api_admin_post_delete(
    State(state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    result_into_response(async move {
        let admin_token = state.primary_secret.derive_secret("admin_token");
        let token = hex::decode(params.get("token").ok_or_else(|| anyhow::anyhow!("Missing token parameter"))?)?;

        if admin_token != token {
            return Err(anyhow::anyhow!("Invalid token"));
        }

        let uuid = params.get("uuid").ok_or_else(|| anyhow::anyhow!("Missing uuid parameter"))?;

        let mut db_connection = state.db_connection.lock().unwrap();
        let trx = db_connection.transaction()?;

        if let Err(_) = trx.query_row("SELECT id FROM post WHERE uuid = ?", [uuid], |row| row.get::<_, u32>(0)) {
            return Err(anyhow::anyhow!("Post not found"));
        }

        trx.execute("UPDATE post SET is_deleted = 1 WHERE uuid = ?", [uuid])?;
        trx.commit()?;

        Ok(Json(serde_json::json!({
            "status": "ok",
        })))
    }, ErrorReporting::Json).await
}
