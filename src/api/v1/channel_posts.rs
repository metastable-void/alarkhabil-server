
use std::sync::Arc;
use std::collections::HashMap;

use axum::{
    extract::{State, Query},
    response::IntoResponse,
    Json,
};

use crate::state::AppState;
use crate::error_reporting::{ErrorReporting, result_into_response};

use crate::api::v1::types::AuthorSummary;


pub async fn api_channel_posts(
    State(state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    result_into_response(async move {
        let channel_uuid = params.get("uuid").ok_or_else(|| anyhow::anyhow!("Missing uuid parameter"))?;

        let mut db_connection = state.db_connection.lock().unwrap();
        let trx = db_connection.transaction()?;

        let mut stmt = trx.prepare(
            "
                SELECT post.uuid, revision.uuid, revision.created_date, revision.title, author.uuid, author.name
                FROM channel, post, revision, author
                WHERE channel.is_deleted = 0 AND post.is_deleted = 0 AND revision.is_deleted = 0 AND author.is_deleted = 0 
                AND channel.id = post.channel_id AND post.id = revision.post_id AND revision.author_id = author_id
                AND channel.uuid = ?
                GROUP BY post.id
                ORDER BY revision.created_date
                DESC LIMIT 1000
            "
        )?;

        let mut rows = stmt.query([channel_uuid])?;
        let mut posts = Vec::new();

        while let Some(row) = rows.next()? {
            let post_uuid: String = row.get(0)?;
            let revision_uuid: String = row.get(1)?;
            let revision_date: u64 = row.get(2)?;
            let title: String = row.get(3)?;
            let author_uuid: String = row.get(4)?;
            let author_name: String = row.get(5)?;

            posts.push(serde_json::json!({
                "post_uuid": post_uuid,
                "revision_uuid": revision_uuid,
                "revision_date": revision_date,
                "title": title,
                "author": AuthorSummary::new(&author_uuid, &author_name),
            }));
        }

        Ok(Json(serde_json::json!(posts)))
    }, ErrorReporting::Json).await
}
