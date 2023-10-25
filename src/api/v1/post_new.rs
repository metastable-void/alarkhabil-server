
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

use crate::api::v1::types::{
    AuthorSummary,
    RevisionInfo,
    PostInfo,
    ChannelSummary,
};


#[derive(Debug, Clone, Serialize, Deserialize)]
struct MsgPostNew {
    command: MustBe!("post_new"),
    channel_uuid: String,
    title: String,
    text: String,
    tags: Vec<String>,
}

pub async fn api_post_new(
    State(state): State<Arc<AppState>>,
    Json(msg): Json<SignedMessage>,
) -> impl IntoResponse {
    result_into_response(async move {
        // verify message
        let public_key = msg.public_key()?.to_owned();
        let msg = msg.verify()?;
        let msg = serde_json::from_slice::<MsgPostNew>(&msg)?;

        let mut db_connection = state.db_connection.lock().unwrap();
        let trx = db_connection.transaction()?;

        let (author_id, author_uuid, author_name) = trx.query_row(
            "SELECT author.id, author.uuid, author.name FROM author, author_public_key WHERE author_public_key.public_key = ? AND author.is_deleted = 0 AND author.id = author_public_key.author_id",
            [&public_key],
            |row| {
                let author_id: u32 = row.get(0)?;
                let author_uuid: String = row.get(1)?;
                let author_name: String = row.get(2)?;
                Ok((author_id, author_uuid, author_name))
            }
        )?;

        let (channel_id, channel_handle, channel_name, channel_lang) = trx.query_row(
            "SELECT channel.id, channel.handle, channel.name, channel.language_code FROM channel, channel_author WHERE channel.uuid = ? AND channel.is_deleted = 0 AND channel.id = channel_author.channel_id AND channel_author.author_id = ?",
            (&msg.channel_uuid, &author_id),
            |row| {
                let channel_id: u32 = row.get(0)?;
                let channel_handle: String = row.get(1)?;
                let channel_name: String = row.get(2)?;
                let channel_lang: String = row.get(3)?;
                Ok((channel_id, channel_handle, channel_name, channel_lang))
            }
        )?;

        let post_uuid = uuid::Uuid::new_v4().to_string();
        trx.execute(
            "INSERT INTO post (uuid, channel_id) VALUES (?, ?)",
            (&post_uuid, &channel_id),
        )?;

        let post_id = trx.query_row(
            "SELECT id FROM post WHERE uuid = ?",
            [&post_uuid],
            |row| {
                let post_id: u32 = row.get(0)?;
                Ok(post_id)
            }
        )?;

        for tag in &msg.tags {
            trx.execute(
                "INSERT INTO post_tag (post_id, tag) VALUES (?, ?)",
                (&post_id, tag),
            )?;
        }

        let created_date = sys_time::get_sys_time_in_secs();
        let revision_uuid = uuid::Uuid::new_v4().to_string();
        trx.execute(
            "INSERT INTO revision (uuid, post_id, author_id, created_date, title, revision_text) VALUES (?, ?, ?, ?, ?, ?)",
            (&revision_uuid, &post_id, &author_id, &created_date, &msg.title, &msg.text),
        )?;

        trx.commit()?;

        let author = AuthorSummary::new(&author_uuid, &author_name);
        let revision = RevisionInfo::new(&revision_uuid, &author, created_date, &msg.title, &msg.text);
        let channel = ChannelSummary::new(&msg.channel_uuid, &channel_handle, &channel_name, &channel_lang);
        let post = PostInfo::new(&post_uuid, &channel, msg.tags, &revision, &author);

        Ok(Json(post))
    }, ErrorReporting::Json).await
}
