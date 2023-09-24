use crate::models::ticket::{ModelController, Ticket};
use crate::models::Dbo;
use crate::Result;
use axum::extract::{Path, State};
use axum::routing::get;
use axum::{Json, Router};

pub fn routes(mc: ModelController) -> Router {
    Router::new()
        .route("/", get(list_tickets).post(create_ticket))
        .route("/:id", get(get_ticket).delete(delete_ticket))
        .with_state(mc)
}
async fn create_ticket(
    State(mc): State<ModelController>,
    Json(ticket): Json<Ticket>,
) -> Result<Json<Dbo<Ticket>>> {
    let ticket = mc.create_ticket(ticket).await?;

    Ok(Json(ticket))
}

async fn list_tickets(State(mc): State<ModelController>) -> Result<Json<Vec<Dbo<Ticket>>>> {
    Ok(Json(mc.list_tickets().await?))
}

async fn get_ticket(
    State(mc): State<ModelController>,
    Path(id): Path<u64>,
) -> Result<Json<Dbo<Ticket>>> {
    Ok(Json(mc.get_ticket(id).await?))
}

async fn delete_ticket(
    State(mc): State<ModelController>,
    Path(id): Path<u64>,
) -> Result<Json<Dbo<Ticket>>> {
    Ok(Json(mc.delete_tickets(id).await?))
}
