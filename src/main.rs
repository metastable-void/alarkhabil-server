use std::env;
use std::net::SocketAddr;
use std::str::FromStr;

use hyper::StatusCode;
use std::sync::{Arc, Mutex};

use std::future::Future;

use axum::{
    extract::State,
    routing::get,
    Router,
    Server,
    http::header,
    response::IntoResponse,
};

use sha2::Sha256;
use hmac::{Hmac, Mac};

use alarkhabil_server::{PrivateKey, SignedMessage};


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

#[derive(Debug)]
struct AppState {
    db_connection: Mutex<rusqlite::Connection>,
    primary_secret: PrimarySecret,
}

async fn handle_anyhow_error(err: anyhow::Error) -> impl IntoResponse {
    log::error!("Error: {}", err);

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        [(header::CONTENT_TYPE, "text/html")],
        "<!doctype html><html><head><title>Internal Server Error</title></head><body><h1>Internal Server Error</h1></body></html>",
    )
}

async fn result_into_response<T: IntoResponse, Fut: Future<Output = anyhow::Result<T>>>(result: Fut) -> impl IntoResponse {
    match result.await {
        Ok(response) => {
            response.into_response()
        },
        Err(e) => {
            handle_anyhow_error(e).await.into_response()
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
    }).await
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

    log::debug!("Length of a derived secret: {}", state.primary_secret.derive_secret("test").len());

    let app = Router::new()
        .route("/", get(get_root))
        .with_state(state.clone());

    let server = Server::bind(&addr).serve(app.into_make_service());

    log::info!("Listening on http://{}", addr);
    server.await?;

    Ok(())
}
