
use std::collections::HashMap;
use std::sync::Arc;

use axum::{
    extract::{State, Query},
    response::IntoResponse,
    Json,
};

use crate::crypto::{PrivateKey, SignedMessage};
use crate::base64;
use crate::state::AppState;
use crate::error_reporting::{ErrorReporting, result_into_response};
use crate::api::v1::types::Invite;


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
