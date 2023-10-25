
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

use crate::api::v1::types::{
    ChannelSummary,
    AuthorSummary,
};


pub async fn api_post_info(
    State(state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    result_into_response(async move {
        let post_uuid = if let Some(post_uuid) = params.get("uuid") {
            post_uuid
        } else {
            return Err(anyhow::anyhow!("Missing uuid parameter"));
        };

        let mut db_connection = state.db_connection.lock().unwrap();
        let trx = db_connection.transaction()?;

        let mut stmt = trx.prepare(
            "SELECT DISTINCT post_tag.name FROM post_tag INNER JOIN post ON post_tag.post_id = post.id WHERE post.is_deleted = 0 AND post.uuid = ?"
        )?;
        let mut tag_rows = stmt.query([post_uuid])?;
        let mut tags: Vec<String> = Vec::new();
        while let Some(tag_row) = tag_rows.next()? {
            let tag: String = tag_row.get(0)?;
            tags.push(tag);
        }

        drop(tag_rows);
        drop(stmt);

        let (
            channel_uuid,
            channel_handle,
            channel_name,
            channel_lang,
            revision_uuid,
            revision_date,
            title,
            author_uuid,
            author_name,
            revision_text,
        ) = if let Ok(values) = trx.query_row(
            "
                SELECT channel.uuid, channel.handle, channel.name, channel.language_code, revision.uuid, revision.created_date, revision.title, author.uuid, author.name, revision.revision_text
                FROM channel, post, revision, author
                WHERE channel.is_deleted = 0 AND post.is_deleted = 0 AND revision.is_deleted = 0 AND author.is_deleted = 0 AND post.uuid = ? AND post.channel_id = channel.id AND post.id = revision.post_id AND revision.author_id = author.id
                ORDER BY revision.created_date DESC LIMIT 1
            ",
            [post_uuid],
            |row| {
                let channel_uuid: String = row.get(0)?;
                let channel_handle: String = row.get(1)?;
                let channel_name: String = row.get(2)?;
                let channel_lang: String = row.get(3)?;
                let revision_uuid: String = row.get(4)?;
                let revision_date: u64 = row.get(5)?;
                let title: String = row.get(6)?;
                let author_uuid: String = row.get(7)?;
                let author_name: String = row.get(8)?;
                let revision_text: String = row.get(9)?;
                Ok((
                    channel_uuid,
                    channel_handle,
                    channel_name,
                    channel_lang,
                    revision_uuid,
                    revision_date,
                    title,
                    author_uuid,
                    author_name,
                    revision_text,
                ))
            }
        ) {
            values
        } else {
            return Ok((
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({
                    "status": "not found",
                })),
            ).into_response());
        };

        let channel = ChannelSummary::new(&channel_uuid, &channel_handle, &channel_name, &channel_lang);
        let author = AuthorSummary::new(&author_uuid, &author_name);

        Ok(Json(serde_json::json!({
            "post_uuid": post_uuid,
            "channel": channel,
            "tags": tags,
            "revision_uuid": revision_uuid,
            "revision_date": revision_date,
            "title": title,
            "revision_text": revision_text,
            "author": author,
        })).into_response())
    }, ErrorReporting::Json).await
}
