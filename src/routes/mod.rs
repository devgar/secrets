mod api;

use axum::routing::{get, get_service};
use axum::Router;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

pub fn routes() -> Router {
    Router::new()
        .route("/", get(root))
        .nest("/api", api::routes())
        .layer(TraceLayer::new_for_http())
        .fallback_service(routes_static())
}

async fn root() -> &'static str {
    "Hello, World!"
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}
