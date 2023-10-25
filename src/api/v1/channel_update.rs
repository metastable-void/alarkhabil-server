
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

use crate::api::v1::types::{
    validate_language_code,
    validate_channel_handle,
    ChannelInfo,
};


#[derive(Debug, Clone, Serialize, Deserialize)]
struct MsgChannelUpdate {
    command: MustBe!("channel_update"),
    uuid: String,
    handle: String,
    name: String,
    lang: String,
    description_text: String,
}

pub async fn api_channel_update(
    State(state): State<Arc<AppState>>,
    Json(msg): Json<SignedMessage>,
) -> impl IntoResponse {
    result_into_response(async move {
        // verify message
        let public_key = msg.public_key()?.to_owned();
        let msg = msg.verify()?;
        let msg = serde_json::from_slice::<MsgChannelUpdate>(&msg)?;

        validate_language_code(&msg.lang)?;
        validate_channel_handle(&msg.handle)?;

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

        let (channel_id, created_date) = trx.query_row(
            "SELECT channel.id, channel.created_date FROM channel, channel_author WHERE channel.uuid = ? AND channel.is_deleted = 0 AND channel.id = channel_author.channel_id AND channel_author.author_id = ?",
            (&msg.uuid, &author_id),
            |row| {
                let channel_id: u32 = row.get(0)?;
                let created_date: u64 = row.get(1)?;
                Ok((channel_id, created_date))
            }
        )?;

        trx.execute(
            "UPDATE channel SET handle = ?, name = ?, language_code = ?, description_text = ? WHERE id = ?",
            (&msg.handle, &msg.name, &msg.lang, &msg.description_text, &channel_id),
        )?;

        trx.commit()?;

        let channel = ChannelInfo::new(&msg.uuid, &msg.handle, &msg.name, created_date, &msg.lang, &msg.description_text);

        Ok(Json(channel))
    }, ErrorReporting::Json).await
}
