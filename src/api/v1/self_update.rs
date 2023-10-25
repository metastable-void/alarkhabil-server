
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

use crate::api::v1::types::AuthorInfo;


#[derive(Debug, Clone, Serialize, Deserialize)]
struct MsgSelfUpdate {
    command: MustBe!("self_update"),
    name: String,
    description_text: String,
}

pub async fn api_self_update(
    State(state): State<Arc<AppState>>,
    Json(msg): Json<SignedMessage>,
) -> impl IntoResponse {
    result_into_response(async move {
        // verify message
        let public_key = msg.public_key()?.to_owned();
        let msg = msg.verify()?;
        let msg = serde_json::from_slice::<MsgSelfUpdate>(&msg)?;

        let mut db_connection = state.db_connection.lock().unwrap();
        let trx = db_connection.transaction()?;

        let (author_id, author_uuid, created_date) = trx.query_row(
            "SELECT author.id, author.uuid, author.created_date FROM author, author_public_key WHERE author_public_key.public_key = ? AND author.id = author_public_key.author_id",
            [&public_key],
            |row| {
                let author_id: u32 = row.get(0)?;
                let author_uuid: String = row.get(1)?;
                let created_date: u64 = row.get(2)?;
                Ok((author_id, author_uuid, created_date))
            }
        )?;
        trx.execute(
            "UPDATE author SET name = ?, description_text = ? WHERE id = ?",
            (&msg.name, &msg.description_text, author_id),
        )?;
        trx.commit()?;

        let author = AuthorInfo::new(&author_uuid, &msg.name, created_date, &msg.description_text);

        Ok(Json(author))
    }, ErrorReporting::Json).await
}
