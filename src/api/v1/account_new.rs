
use std::sync::Arc;

use serde::{Serialize, Deserialize};
use monostate::MustBe;

use axum::{
    extract::State,
    response::IntoResponse,
    Json,
};

use crate::crypto::{PrivateKey, SignedMessage};
use crate::sys_time;
use crate::base64;
use crate::state::AppState;
use crate::error_reporting::{ErrorReporting, result_into_response};
use crate::api::v1::types::Invite;
use crate::limits;


#[derive(Debug, Clone, Serialize, Deserialize)]
struct MsgAccountNew {
    command: MustBe!("account_new"),
    name: String,
    invite: String,
}

pub async fn api_account_new(
    State(state): State<Arc<AppState>>,
    Json(msg): Json<SignedMessage>,
) -> impl IntoResponse {
    result_into_response(async move {
        let public_key = msg.public_key()?.to_owned();
        let msg = msg.verify()?;
        let msg = serde_json::from_slice::<MsgAccountNew>(&msg)?;
        let invite = base64::decode(&msg.invite)?;
        let signing_key = state.primary_secret.derive_secret("signing_key");
        let invite: SignedMessage = serde_json::from_slice(&invite)?;

        let key = PrivateKey::from_bytes("hmac-sha256", &signing_key)?;
        let invite_msg = invite.verify_with_secret(key)?;
        let invite_msg = serde_json::from_slice::<Invite>(&invite_msg)?;

        if msg.name.len() > limits::MAX_ITEM_NAME_SIZE {
            return Err(anyhow::anyhow!("Name is too long"));
        }

        let uuid = invite_msg.uuid();

        if let Ok(uuid) = uuid::Uuid::parse_str(uuid) {
            if uuid.get_version_num() != 4 {
                return Err(anyhow::anyhow!("Invalid invite"));
            }
        } else {
            return Err(anyhow::anyhow!("Invalid invite"));
        };

        let mut db_connection = state.db_connection.lock().unwrap();
        let trx = db_connection.transaction()?;

        if let Ok(_) = trx.query_row("SELECT id FROM author WHERE uuid = ?", [uuid], |row| row.get::<_, u32>(0)) {
            return Err(anyhow::anyhow!("Author already exists"));
        }

        let name = &msg.name;
        let now = sys_time::get_sys_time_in_secs();

        trx.execute("INSERT INTO author (uuid, name, registered_date) VALUES (?, ?, ?)", (uuid, name, now))?;
        let author_id = trx.query_row("SELECT id FROM author WHERE uuid = ?", [uuid], |row| row.get::<_, u32>(0))?;

        trx.execute("INSERT INTO author_public_key (author_id, type, public_key) VALUES (?, ?, ?)", (author_id, "ed25519", public_key))?;

        trx.commit()?;

        Ok(Json(serde_json::json!({
            "status": "ok",
            "uuid": uuid,
        })))
    }, ErrorReporting::Json).await
}
