
use std::collections::HashMap;
use std::sync::Arc;

use axum::{
    extract::{State, Query},
    response::IntoResponse,
    Json,
};

use crate::state::AppState;
use crate::error_reporting::{ErrorReporting, result_into_response};


pub async fn api_admin_meta_delete(
    State(state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    result_into_response(async move {
        let admin_token = state.primary_secret.derive_secret("admin_token");
        let token = hex::decode(params.get("token").ok_or_else(|| anyhow::anyhow!("Missing token parameter"))?)?;

        // TODO: prevent time-based attacks
        if admin_token != token {
            return Err(anyhow::anyhow!("Invalid token"));
        }

        let page_name = params.get("page_name").ok_or_else(|| anyhow::anyhow!("Missing page_name parameter"))?;

        let mut db_connection = state.db_connection.lock().unwrap();
        let trx = db_connection.transaction()?;

        trx.execute("DELETE FROM meta_page WHERE page_name = ?", [page_name])?;
        trx.commit()?;

        Ok(Json(serde_json::json!({
            "status": "ok",
        })))
    }, ErrorReporting::Json).await
}
