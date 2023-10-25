
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
use crate::sys_time;

use crate::api::v1::types::{validate_channel_handle, ChannelInfo};


#[derive(Debug, Clone, Serialize, Deserialize)]
struct MsgChannelNew {
    command: MustBe!("channel_new"),
    handle: String,
    name: String,
    lang: String,
}

pub async fn api_channel_new(
    State(state): State<Arc<AppState>>,
    Json(msg): Json<SignedMessage>,
) -> impl IntoResponse {
    result_into_response(async move {
        // verify message
        let public_key = msg.public_key()?.to_owned();
        let msg = msg.verify()?;
        let msg = serde_json::from_slice::<MsgChannelNew>(&msg)?;

        if msg.lang.len() > 64 {
            return Err(anyhow::anyhow!("Invalid language code"));
        }

        validate_channel_handle(&msg.handle)?;

        let mut db_connection = state.db_connection.lock().unwrap();
        let trx = db_connection.transaction()?;

        let author_id = trx.query_row(
            "SELECT author.id FROM author, author_public_key WHERE author_public_key.public_key = ? AND author.id = author_public_key.author_id",
            [&public_key],
            |row| {
                let author_id: u32 = row.get(0)?;
                Ok(author_id)
            }
        )?;

        let uuid = uuid::Uuid::new_v4().to_string();
        let created_date = sys_time::get_sys_time_in_secs();
        trx.execute(
            "INSERT INTO channel (uuid, handle, name, created_date, language_code) VALUES (?, ?, ?, ?, ?)",
            (&uuid, &msg.handle, &msg.name, created_date, &msg.lang),
        )?;

        let channel_id = trx.query_row(
            "SELECT id FROM channel WHERE uuid = ?",
            [&uuid],
            |row| {
                let channel_id: u32 = row.get(0)?;
                Ok(channel_id)
            }
        )?;

        trx.execute(
            "INSERT INTO channel_author (channel_id, author_id) VALUES (?, ?)",
            (channel_id, author_id),
        )?;
        
        trx.commit()?;

        let channel = ChannelInfo::new(&uuid, &msg.handle, &msg.name, created_date, &msg.lang, "");

        Ok(Json(channel))
    }, ErrorReporting::Json).await
}
