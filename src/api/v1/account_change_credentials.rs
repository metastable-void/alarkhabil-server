
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
struct MsgAccountChangeCredentials {
    command: MustBe!("account_change_credentials"),
    new_algo: String,

    #[serde(with="crate::base64")]
    new_public_key: Vec<u8>,

    #[serde(with="crate::base64")]
    signature: Vec<u8>,
}

pub async fn api_account_change_credentials(
    State(state): State<Arc<AppState>>,
    Json(msg): Json<SignedMessage>,
) -> impl IntoResponse {
    result_into_response(async move {
        // verify message
        let public_key = msg.public_key()?.to_owned();
        let msg = msg.verify()?;
        let msg = serde_json::from_slice::<MsgAccountChangeCredentials>(&msg)?;

        if &msg.new_algo != "ed25519" {
            return Err(anyhow::anyhow!("Unsupported algorithm: {}", &msg.new_algo));
        }

        let signed_msg = SignedMessage::try_new(&msg.new_algo, &msg.new_public_key, &msg.signature, &public_key)?;

        signed_msg.verify().map_err(|_| anyhow::anyhow!("Invalid signature"))?;

        let mut db_connection = state.db_connection.lock().unwrap();
        let trx = db_connection.transaction()?;

        let public_key_id = trx.query_row("SELECT id FROM author_public_key WHERE public_key = ?", [&public_key], |row| row.get::<_, u32>(0))?;
        trx.execute("UPDATE author_public_key SET public_key = ? WHERE id = ?", (&msg.new_public_key, public_key_id))?;
        trx.commit()?;

        Ok(Json(serde_json::json!({
            "status": "ok",
        })))
    }, ErrorReporting::Json).await
}
