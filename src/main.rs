mod error;
mod models;
mod routes;

pub use crate::error::{Error, Result};

use axum::http::header;
use axum::http::HeaderName;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let app = routes::routes();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
fn create_location(name: &str, id: u64) -> (HeaderName, &'static str) {
    (
        header::LOCATION,
        Box::leak(format!("/{}/{}", name, id).into_boxed_str()),
    )
}
