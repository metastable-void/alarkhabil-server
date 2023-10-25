
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
struct MsgChannelDelete {
    command: MustBe!("channel_delete"),
    uuid: String,
}

pub async fn api_channel_delete(
    State(state): State<Arc<AppState>>,
    Json(msg): Json<SignedMessage>,
) -> impl IntoResponse {
    result_into_response(async move {
        // verify message
        let public_key = msg.public_key()?.to_owned();
        let msg = msg.verify()?;
        let msg = serde_json::from_slice::<MsgChannelDelete>(&msg)?;

        let mut db_connection = state.db_connection.lock().unwrap();
        let trx = db_connection.transaction()?;

        let author_id = trx.query_row(
            "SELECT author.id FROM author, author_public_key WHERE author_public_key.public_key = ? AND author.is_deleted = 0 AND author.id = author_public_key.author_id",
            [&public_key],
            |row| {
                let author_id: u32 = row.get(0)?;
                Ok(author_id)
            }
        )?;

        let channel_id = trx.query_row(
            "SELECT channel.id FROM channel, channel_author WHERE channel.uuid = ? AND channel.is_deleted = 0 AND channel.id = channel_author.channel_id AND channel_author.author_id = ?",
            (&msg.uuid, &author_id),
            |row| {
                let channel_id: u32 = row.get(0)?;
                Ok(channel_id)
            }
        )?;

        trx.execute(
            "UPDATE channel SET is_deleted = 1 WHERE id = ?",
            [&channel_id],
        )?;

        trx.commit()?;

        Ok(Json(serde_json::json!({
            "status": "ok",
        })))
    }, ErrorReporting::Json).await
}
