use std::env;
use std::net::SocketAddr;
use std::str::FromStr;

use std::collections::HashMap;

use hyper::StatusCode;
use std::sync::{Arc, Mutex};

use std::future::Future;

use serde::{Serialize, Deserialize};

use axum::{
    extract::{State, Query},
    routing::{get, post},
    Router,
    Server,
    http::header,
    response::IntoResponse,
    Json,
};

use sha2::Sha256;
use hmac::{Hmac, Mac};
use base64::{Engine, engine::general_purpose::STANDARD as base64_engine};

use alarkhabil_server::{PrivateKey, SignedMessage, NewInviteResult, get_sys_time_in_secs};


type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, Clone)]
struct PrimarySecret {
    secret: String,
}

impl PrimarySecret {
    fn derive_secret(&self, name: &str) -> Vec<u8> {
        let mut hmac = HmacSha256::new_from_slice(self.secret.as_bytes()).unwrap();
        hmac.update(name.as_bytes());
        hmac.finalize().into_bytes().to_vec()
    }
}

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

#[derive(Debug)]
struct AppState {
    db_connection: Mutex<rusqlite::Connection>,
    primary_secret: PrimarySecret,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
enum ErrorReporting {
    Html,
    Json,
}

async fn handle_anyhow_error(err: anyhow::Error, reporting: ErrorReporting) -> impl IntoResponse {
    log::error!("Error: {}", err);

    if ErrorReporting::Json == reporting {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "status": "error",
                "message": err.to_string(),
            })),
        ).into_response();
    }

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        [(header::CONTENT_TYPE, "text/html")],
        "<!doctype html><html><head><title>Internal Server Error</title></head><body><h1>Internal Server Error</h1></body></html>",
    ).into_response()
}

async fn result_into_response<T: IntoResponse, Fut: Future<Output = anyhow::Result<T>>>(
    result: Fut,
    error_reporting: ErrorReporting,
) -> impl IntoResponse {
    match result.await {
        Ok(response) => {
            response.into_response()
        },
        Err(e) => {
            handle_anyhow_error(e, error_reporting).await.into_response()
        }
    }
}

async fn get_root(
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    result_into_response(async move {
        let author_count: u32 = {
            let db_connection = state.db_connection.lock().unwrap();
            db_connection.query_row("SELECT COUNT(*) FROM author", [], |row| {
                row.get(0)
            })?
        };
    
        let secret_key = PrivateKey::new("ed25519")?;
        let msg = b"Hello, world!";
        let signed_msg = SignedMessage::create(secret_key, msg)?;
        signed_msg.verify()?;
    
        Ok(format!("Hello, world! {} authors", author_count))
    }, ErrorReporting::Html).await
}

async fn api_invite_new(
    State(state): State<Arc<AppState>>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    result_into_response(async move {
        let invite_token = state.primary_secret.derive_secret("new_invite_token");
        let token = hex::decode(params.get("token").ok_or_else(|| anyhow::anyhow!("Missing token parameter"))?)?;

        if invite_token != token {
            return Err(anyhow::anyhow!("Invalid token"));
        }

        let signing_key = state.primary_secret.derive_secret("signing_key");

        let msg = Invite::new();
        let msg = SignedMessage::create(PrivateKey::from_bytes("hmac-sha256", &signing_key)?, serde_json::to_string(&msg)?.as_bytes())?;
        let msg = NewInviteResult {
            invite: serde_json::to_string(&msg)?.as_bytes().to_vec(),
        };

        Ok(Json(msg))
    }, ErrorReporting::Json).await
}

async fn api_account_new(
    State(state): State<Arc<AppState>>,
    Json(msg): Json<SignedMessage>,
) -> impl IntoResponse {
    result_into_response(async move {
        let public_key = msg.public_key()?.to_owned();
        let msg = msg.verify()?;
        let msg = serde_json::from_slice::<MsgAccountNew>(&msg)?;
        let invite = base64_engine.decode(&msg.invite)?;
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
        let now = get_sys_time_in_secs();

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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;
    env_logger::init();

    // bind address
    let addr_string = env::var("LISTEN_ADDR").unwrap_or("".to_string());
    let addr = SocketAddr::from_str(&addr_string).unwrap_or(SocketAddr::from(([127, 0, 0, 1], 8080)));

    log::debug!("Bind address: {}", addr);

    // SQLite path
    let db_path: String = env::var("DB_PATH").unwrap_or("".to_string());

    // initialize DB
    let mut db_connection = if db_path.is_empty() {
        log::warn!("DB_PATH not set, using in-memory database");
        rusqlite::Connection::open_in_memory()?
    } else {
        log::info!("Using database at {}", db_path);
        rusqlite::Connection::open(&db_path)?
    };
    {
        let init_query = include_str!("./sql/schema-sqlite.sql");
        let init_tx = db_connection.transaction()?;
        init_tx.execute_batch(init_query)?;
        init_tx.commit()?;
    }

    // run the server
    let state = Arc::new(AppState {
        db_connection: Mutex::new(db_connection),
        primary_secret: PrimarySecret {
            secret: env::var("PRIMARY_SECRET").unwrap_or_else(|_| {
                log::warn!("PRIMARY_SECRET not set, using temporary random value");
                let buf = rand::random::<[u8; 32]>();
                hex::encode(buf)
            }),
        },
    });

    let new_invite_token = state.primary_secret.derive_secret("new_invite_token");

    let app = Router::new()
        .route("/", get(get_root))
        .route("/api/v1/invite/new", get(api_invite_new))
        .route("/api/v1/account/new", post(api_account_new))
        .with_state(state.clone());

    let server = Server::bind(&addr).serve(app.into_make_service());

    log::info!("Listening on http://{}", &addr);
    println!("New invite tokens available at: http://{}/api/v1/invite/new?token={}", &addr, hex::encode(new_invite_token));

    server.await?;

    Ok(())
}
