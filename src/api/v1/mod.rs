
pub mod types;


use std::collections::HashMap;
use std::sync::Arc;

use serde::{Serialize, Deserialize};

use axum::{
    extract::{State, Query},
    response::IntoResponse,
    Json,
};

use crate::crypto::{PrivateKey, SignedMessage};
use crate::sys_time;
use crate::base64;
use crate::state::AppState;
use crate::error_reporting::{ErrorReporting, result_into_response};


#[derive(Debug, Clone, Serialize, Deserialize)]
struct Invite {
    command: String,
    uuid: String,
}

impl Invite {
    fn new() -> Invite {
        let uuid = uuid::Uuid::new_v4().to_string();
        Invite {
            command: "registration_invite".to_string(),
            uuid,
        }
    }

    fn uuid(&self) -> &str {
        &self.uuid
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MsgAccountNew {
    name: String,
    invite: String,
}

pub async fn api_invite_new(
    State(state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    result_into_response(async move {
        let invite_making_token = state.primary_secret.derive_secret("invite_making_token");
        let token = hex::decode(params.get("token").ok_or_else(|| anyhow::anyhow!("Missing token parameter"))?)?;

        if invite_making_token != token {
            return Err(anyhow::anyhow!("Invalid token"));
        }

        let signing_key = state.primary_secret.derive_secret("signing_key");

        let msg = Invite::new();
        let msg = SignedMessage::create(PrivateKey::from_bytes("hmac-sha256", &signing_key)?, serde_json::to_string(&msg)?.as_bytes())?;

        let invite_token = base64::encode(serde_json::to_string(&msg)?.as_bytes());

        Ok(Json(serde_json::json!({
            "status": "ok",
            "invite": invite_token,
        })))
    }, ErrorReporting::Json).await
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

        let uuid = invite_msg.uuid();

        let mut db_connection = state.db_connection.lock().unwrap();
        let trx = db_connection.transaction()?;

        if let Ok(_) = trx.query_row("SELECT id FROM author WHERE uuid = ?", [uuid], |row| row.get::<_, u32>(0)) {
            return Err(anyhow::anyhow!("Author already exists"));
        }

        let name = &msg.name;
        let now = sys_time::get_sys_time_in_secs();

        trx.execute("INSERT INTO author (uuid, name, registered_date) VALUES (?, ?, ?)", (uuid, name, now))?;
        let author_id = trx.query_row("SELECT id FROM author WHERE uuid = ?", [uuid], |row| row.get::<_, u32>(0))?;

        trx.execute("INSERT INTO public_key (author_id, type, public_key) VALUES (?, ?, ?)", (author_id, "ed25519", public_key))?;

        trx.commit()?;

        Ok(Json(serde_json::json!({
            "status": "ok",
            "uuid": uuid,
        })))
    }, ErrorReporting::Json).await
}
