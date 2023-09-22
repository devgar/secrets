mod error;
mod web;

pub use crate::error::{Error, Result};

use axum::http::HeaderName;
use axum::{
    extract::Path,
    http::{header, StatusCode},
    routing::{delete, get, post},
    Json, Router,
};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .merge(web::routes_login::routes())
        .route("/", get(root))
        .route("/users", post(create_user))
        .route("/users/:id", delete(delete_user));

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

async fn root() -> &'static str {
    tracing::info!("{:>8} /", "GET");
    "Hello, World!"
}

async fn create_user(
    Json(payload): Json<User>,
) -> (
    StatusCode,
    [(HeaderName, &'static str); 1],
    // Json<Option<DBO<User>>>,
) {
    tracing::info!("{:>8} /users", "POST");
    // insert your application logic here
    let user = DBO::insert(payload);

    (
        // Response CREATED (LOCATION=/users/:userId
        StatusCode::CREATED,
        [create_location("users", user.id)],
        // Json(Some(user)),
    )
}

async fn delete_user(Path(id): Path<u64>) -> (StatusCode, Json<DBO<User>>) {
    tracing::info!("{:>8} /users/{}", "DELETE", id);
    (
        StatusCode::OK,
        Json(DBO {
            id,
            entity: User {
                username: String::from("Gar"),
            },
        }),
    )
}

#[derive(Deserialize, Serialize)]
struct DBO<T: Serialize> {
    id: u64,
    #[serde(flatten)]
    entity: T,
}

impl<T: Serialize> DBO<T> {
    fn insert(payload: T) -> DBO<T> {
        let mut rng = rand::thread_rng();
        // DO SOMETHING TO INSERT IN DATABASE
        DBO {
            id: rng.gen_range(1..999),
            entity: payload,
        }
    }
}

// the input to our `create_user` handler
#[derive(Deserialize, Serialize)]
struct User {
    username: String,
}

impl User {
    const TABLE: &'static str = "users";
}
