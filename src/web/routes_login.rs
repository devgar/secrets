use std::ffi::CString;
use axum::{Json, Router};
use axum::routing::post;
use serde::Deserialize;
use serde_json::{json, Value};
use crate::{Error, Result};

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}
async fn api_login(payload: Json<LoginPayload>) -> Result<Json<Value>> {
    tracing::info!("->> {:12} - api_login", "HANDLER");

    if payload.username != "demo1" || payload.password != "welcome" {
        return Err(Error::LoginFail);
    }

    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)

    // todo!()
}
#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    password: String,
}