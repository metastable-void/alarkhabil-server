
use std::sync::Arc;

use serde::{Serialize, Deserialize};
use monostate::MustBe;

use axum::{
    extract::State,
    response::IntoResponse,
    Json,
};

use crate::crypto::SignedMessage;
use crate::state::AppState;
use crate::error_reporting::{ErrorReporting, result_into_response};


#[derive(Debug, Clone, Serialize, Deserialize)]
struct MsgAccountDelete {
    command: MustBe!("account_delete"),
}

pub async fn api_account_delete(
    State(state): State<Arc<AppState>>,
    Json(msg): Json<SignedMessage>,
) -> impl IntoResponse {
    result_into_response(async move {
        // verify message
        let public_key = msg.public_key()?.to_owned();
        let msg = msg.verify()?;
        serde_json::from_slice::<MsgAccountDelete>(&msg)?;

        let mut db_connection = state.db_connection.lock().unwrap();
        let trx = db_connection.transaction()?;

        let id = trx.query_row("SELECT author.id FROM author, author_public_key WHERE author_public_key.public_key = ? AND author.id = author_public_key.author_id", [&public_key], |row| row.get::<_, u32>(0))?;
        trx.execute("UPDATE author SET is_deleted = 1 WHERE id = ?", [id])?;
        trx.commit()?;

        Ok(Json(serde_json::json!({
            "status": "ok",
        })))
    }, ErrorReporting::Json).await
}
