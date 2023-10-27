
use std::sync::Arc;
use std::collections::HashMap;

use axum::{
    extract::{State, Query},
    response::IntoResponse,
    Json,
};

use crate::state::AppState;
use crate::error_reporting::{ErrorReporting, result_into_response};


pub async fn api_tag_list(
    State(state): State<Arc<AppState>>,
    Query(_params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    result_into_response(async move {
        let mut db_connection = state.db_connection.lock().unwrap();
        let trx = db_connection.transaction()?;

        let mut stmt = trx.prepare(
            "
                SELECT post_tag.name, COUNT(post.id)
                FROM channel, post, post_tag
                WHERE channel.is_deleted = 0 AND post.is_deleted = 0
                AND channel.id = post.channel_id AND post_tag.post_id = post.id
                GROUP BY post_tag.name
                ORDER BY COUNT(post.id) DESC
                LIMIT 1000
            "
        )?;

        let mut rows = stmt.query([])?;
        let mut tags = Vec::new();

        while let Some(row) = rows.next()? {
            let tag_name: String = row.get(0)?;
            let page_count: u64 = row.get(1)?;

            tags.push(serde_json::json!({
                "tag_name": tag_name,
                "page_count": page_count,
            }));
        }

        Ok(Json(serde_json::json!(tags)))
    }, ErrorReporting::Json).await
}
