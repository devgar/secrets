use axum::routing::{get, get_service};
use axum::Router;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

mod routes_login;
mod routes_users;

pub fn routes() -> Router {
    Router::new()
        .route("/", get(root))
        .nest("/users", routes_users::routes())
        .nest("/login", routes_login::routes())
        .layer(TraceLayer::new_for_http())
        .fallback_service(routes_static())
}

async fn root() -> &'static str {
    "Hello, World!"
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}
