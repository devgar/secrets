use crate::{create_location, models};
use axum::extract::Path;
use axum::http::{HeaderName, StatusCode};
use axum::routing::{delete, post};
use axum::{Json, Router};

pub fn routes() -> Router {
    Router::new()
        .route("/", post(create_user))
        .route("/:id", delete(delete_user))
}

pub async fn delete_user(Path(id): Path<u64>) -> (StatusCode, Json<models::Dbo<models::User>>) {
    (StatusCode::OK, Json(models::Dbo::<models::User>::get(id)))
}

async fn create_user(
    Json(payload): Json<models::User>,
) -> (
    StatusCode,
    [(HeaderName, &'static str); 1],
    // Json<Option<DBO<User>>>,
) {
    // insert your application logic here
    let user = models::Dbo::insert(payload);

    (
        // Response CREATED (LOCATION=/users/:userId
        StatusCode::CREATED,
        [create_location("users", user.id)],
        // Json(Some(user)),
    )
}
