use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    LoginFail,
}
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        tracing::error!("--{:<12} - {self:?}", "INTO_RES");

        //todo!()
        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
    }
}