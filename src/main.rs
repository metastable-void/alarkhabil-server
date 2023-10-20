use std::env;
use std::net::SocketAddr;
use std::str::FromStr;

use std::convert::Infallible;
use hyper::server::conn::AddrStream;
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use hyper::StatusCode;
use std::sync::{Arc, Mutex};

use sha2::Sha256;
use hmac::{Hmac, Mac};


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

async fn handle_request(_req: &mut Request<Body>, state: Arc<AppState>) -> anyhow::Result<Response<Body>> {
    let author_count: u32 = {
        let db_connection = state.db_connection.lock().unwrap();
        db_connection.query_row("SELECT COUNT(*) FROM author", [], |row| {
            row.get(0)
        })?
    };

    Ok(
        Response::builder()
            .status(StatusCode::OK)
            .body(Body::from(format!("Hello, world! {} authors", author_count)))
            .unwrap()
    )
}

async fn process_request(mut req: Request<Body>, _addr: SocketAddr, state: Arc<AppState>) -> Result<Response<Body>, Infallible> {
    let result = handle_request(&mut req, state).await;

    let response = match result {
        Ok(response) => {
            response
        },
        Err(e) => {
            log::error!("Error: {}", e);
            let response = Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("Internal Server Error"))
                .unwrap();
            response
        }
    };

    log::debug!("[{}] {} {}", response.status(), req.method(), req.uri());
    Ok(response)
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

    let make_svc = make_service_fn(|conn: &AddrStream| {
        let addr = conn.remote_addr();
        let state = state.clone();
        async move {
            let addr = addr.clone();
            Ok::<_, Infallible>(service_fn(move |req : Request<Body>| {
                process_request(req, addr, state.clone())
            }))
        }
    });

    let server = Server::bind(&addr).serve(make_svc);

    log::info!("Listening on http://{}", addr);
    server.await?;

    Ok(())
}
