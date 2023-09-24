use axum::Router;

mod routes_login;
mod routes_users;

pub fn routes() -> Router {
    Router::new()
        .nest("/users", routes_users::routes())
        .nest("/login", routes_login::routes())
}
