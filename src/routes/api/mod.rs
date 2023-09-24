use crate::models::ticket::ModelController;
use axum::Router;

mod routes_login;
mod routes_tickets;
mod routes_users;

pub fn routes(mc: ModelController) -> Router {
    Router::new()
        .nest("/users", routes_users::routes())
        .nest("/login", routes_login::routes())
        .nest("/tickets", routes_tickets::routes(mc))
}
