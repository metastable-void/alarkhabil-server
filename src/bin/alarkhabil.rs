
use std::env;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

use hyper::{Request, StatusCode};
use axum::{
    routing::{get, post},
    Router,
    Server,
    response::{IntoResponse, Redirect, Response},
    Json,
    middleware::Next,
};

use alarkhabil_server::state::{PrimarySecret, AppState};

use alarkhabil_server::api;


// const
static SQL_SCHEMA_SQLITE: &str = include_str!("../sql/schema-sqlite.sql");
static URL_GITHUB: &str = "https://github.com/metastable-void/alarkhabil-server";
static RESPONSE_HEADER_CSP: &str = "default-src 'none'; base-uri 'none'; form-action 'none'; frame-ancestors 'none';";
static RESPONSE_HEADER_ACCESS_CONTROL_ALLOW_ORIGIN: &str = "*";


async fn handler_root() -> impl IntoResponse {
    Redirect::to(URL_GITHUB)
}

async fn handler_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Json(serde_json::json!({
            "status": "error",
            "message": "Not found",
        })),
    )
}

async fn add_global_headers<B>(req: Request<B>, next: Next<B>) -> Response {
    let mut res = next.run(req).await;
    let headers = res.headers_mut();
    headers.append("content-security-policy", RESPONSE_HEADER_CSP.parse().unwrap());
    headers.append("access-control-allow-origin", RESPONSE_HEADER_ACCESS_CONTROL_ALLOW_ORIGIN.parse().unwrap());
    res
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
    let addr = SocketAddr::from_str(&addr_string).unwrap_or(SocketAddr::from(([127, 0, 0, 1], 7781)));

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
        .route("/", get(handler_root))

        // Invites v1
        .route("/api/v1/invite/new", get(api::v1::api_invite_new))

        // Accounts v1
        .route("/api/v1/account/new", post(api::v1::api_account_new))
        .route("/api/v1/account/change_credentials", post(api::v1::api_account_change_credentials))
        .route("/api/v1/account/delete", post(api::v1::api_account_delete))

        // Admin v1
        .route("/api/v1/admin/meta/update", post(api::v1::api_admin_meta_update))
        .route("/api/v1/admin/meta/delete", post(api::v1::api_admin_meta_delete))
        .route("/api/v1/admin/author/delete", post(api::v1::api_admin_author_delete))
        .route("/api/v1/admin/channel/delete", post(api::v1::api_admin_channel_delete))
        .route("/api/v1/admin/post/delete", post(api::v1::api_admin_post_delete))

        // Author's endpoints v1
        .route("/api/v1/self/update", post(api::v1::api_self_update))
        .route("/api/v1/channel/new", post(api::v1::api_channel_new))
        .route("/api/v1/channel/update", post(api::v1::api_channel_update))
        .route("/api/v1/channel/delete", post(api::v1::api_channel_delete))
        .route("/api/v1/post/new", post(api::v1::api_post_new))
        .route("/api/v1/post/update", post(api::v1::api_post_update))
        .route("/api/v1/post/delete", post(api::v1::api_post_delete))

        // Public endpoints v1
        .route("/api/v1/meta/info", get(api::v1::api_meta_info))
        .route("/api/v1/meta/list", get(api::v1::api_meta_list))

        // 404 page
        .fallback(handler_404)

        .layer(axum::middleware::from_fn(add_global_headers))

        // Set state
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
