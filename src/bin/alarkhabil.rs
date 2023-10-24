
use std::env;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

use axum::{
    extract::State,
    routing::{get, post},
    Router,
    Server,
    response::IntoResponse,
};

use alarkhabil_server::crypto::{PrivateKey, SignedMessage};
use alarkhabil_server::state::{PrimarySecret, AppState};
use alarkhabil_server::error_reporting::{ErrorReporting, result_into_response};

use alarkhabil_server::api;


// const
static SQL_SCHEMA_SQLITE: &str = include_str!("../sql/schema-sqlite.sql");


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

async fn open_database(db_path: &str) -> Result<rusqlite::Connection, anyhow::Error> {
    let conn = if db_path.is_empty() {
        log::warn!("DB_PATH not set, using in-memory database");
        rusqlite::Connection::open_in_memory()?
    } else {
        log::info!("Using database at {}", db_path);
        rusqlite::Connection::open(&db_path)?
    };
    Ok(conn)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv()?;
    env_logger::init();

    // bind address
    let addr_string = env::var("LISTEN_ADDR").unwrap_or("".to_string());
    let addr = SocketAddr::from_str(&addr_string).unwrap_or(SocketAddr::from(([127, 0, 0, 1], 8080)));

    // SQLite path
    let db_path: String = env::var("DB_PATH").unwrap_or("".to_string());

    // initialize DB
    let mut db_connection = open_database(&db_path).await?;
    {
        let init_tx = db_connection.transaction()?;
        init_tx.execute_batch(SQL_SCHEMA_SQLITE)?;
        init_tx.commit()?;
    }

    // initialize state
    let primary_secret = PrimarySecret::new_from_env();
    let state = Arc::new(AppState {
        db_connection: Mutex::new(db_connection),
        primary_secret: primary_secret,
    });

    // define routes
    let app = Router::new()
        .route("/", get(get_root))
        .route("/api/v1/invite/new", get(api::v1::api_invite_new))
        .route("/api/v1/account/new", post(api::v1::api_account_new))
        .route("/api/v1/admin/author/delete", post(api::v1::api_admin_author_delete))
        .with_state(state.clone());

    // run server
    let server = Server::bind(&addr).serve(app.into_make_service());

    log::info!("Listening on http://{}", &addr);

    // print tokens for admin
    let primary_secret = &state.primary_secret;
    let invite_making_token = primary_secret.derive_secret("invite_making_token");
    let admin_token = primary_secret.derive_secret("admin_token");

    println!("Invite making token: {}", hex::encode(invite_making_token));
    println!("Admin token: {}", hex::encode(admin_token));

    server.await?;

    Ok(())
}
