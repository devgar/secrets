mod api;

use crate::models::ticket::ModelController;
use axum::routing::{get, get_service};
use axum::Router;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

pub fn routes(mc: ModelController) -> Router {
    Router::new()
        .route("/", get(root))
        .nest("/api", api::routes(mc))
        .layer(TraceLayer::new_for_http())
        .fallback_service(routes_static())
}

async fn root() -> &'static str {
    "Hello, World!"
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}
