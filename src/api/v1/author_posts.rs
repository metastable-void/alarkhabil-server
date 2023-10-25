
use std::sync::Arc;
use std::collections::HashMap;

use axum::{
    extract::{State, Query},
    response::IntoResponse,
    Json,
};

use crate::state::AppState;
use crate::error_reporting::{ErrorReporting, result_into_response};

use crate::api::v1::types::ChannelSummary;


pub async fn api_author_posts(
    State(state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    result_into_response(async move {
        let author_uuid = params.get("uuid").ok_or_else(|| anyhow::anyhow!("Missing uuid parameter"))?;

        let mut db_connection = state.db_connection.lock().unwrap();
        let trx = db_connection.transaction()?;

        let mut stmt = trx.prepare(
            "SELECT channel.uuid, channel.handle, channel.name, channel.language_code, post.uuid, revision.uuid, revision.created_date, revision.title FROM channel, post, revision, author WHERE channel.is_deleted = 0 AND post.is_deleted = 0 AND revision.is_deleted = 0 AND channel.id = post.channel_id AND post.id = revision.post_id AND revision.author_id = author.id AND author.is_deleted = 0 AND author.uuid = ? ORDER BY revision.created_date DESC LIMIT 1000"
        )?;

        let mut rows = stmt.query([author_uuid])?;
        let mut posts = Vec::new();

        while let Some(row) = rows.next()? {
            let channel_uuid: String = row.get(0)?;
            let handle: String = row.get(1)?;
            let name: String = row.get(2)?;
            let language_code: String = row.get(3)?;

            let post_uuid: String = row.get(4)?;
            let revision_uuid: String = row.get(5)?;
            let revision_date: u64 = row.get(6)?;
            let title: String = row.get(7)?;

            let channel = ChannelSummary::new(&channel_uuid, &handle, &name, &language_code);
            posts.push(serde_json::json!({
                "post_uuid": post_uuid,
                "revision_uuid": revision_uuid, // this might not be latest revision
                "revision_date": revision_date, // this might not be latest revision
                "title": title, // this might not be latest revision
                "channel": channel,
            }));
        }

        Ok(Json(serde_json::json!(posts)))
    }, ErrorReporting::Json).await
}
