use std::env;
use std::net::SocketAddr;
use std::str::FromStr;

use std::convert::Infallible;
use hyper::server::conn::AddrStream;
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use hyper::StatusCode;


async fn handle_request(req: Request<Body>, _addr: SocketAddr) -> Result<Response<Body>, Infallible> {
    log::debug!("Request: {} {}", req.method(), req.uri());
    Ok(
        Response::builder()
        .status(StatusCode::OK)
        .body(Body::from("Hello, world!"))
        .unwrap()
    )
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
        log::info!("Using in-memory database");
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
    let make_svc = make_service_fn(move |conn: &AddrStream| {
        let addr = conn.remote_addr();
        async move {
            let addr = addr.clone();
            Ok::<_, Infallible>(service_fn(move |req : Request<Body>| {
                handle_request(req, addr)
            }))
        }
    });

    let server = Server::bind(&addr).serve(make_svc);

    server.await?;

    Ok(())
}
