use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    LoginFail,

    // -- Model errors
    TicketIdNotFound { id: u64 },
}
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        #![allow(unreachable_patterns)]
        (match self {
            Error::LoginFail => (StatusCode::BAD_REQUEST, "WRONG_USERNAME_OR_PASSWORD"),
            Error::TicketIdNotFound { id } => (
                StatusCode::NOT_FOUND,
                &*(format!("TICKET_WID_ID_{}_NOT_FOUND", id).leak()),
            ),
            _ => ( // COULD BE UNREACHABLE
                StatusCode::INTERNAL_SERVER_ERROR,
                "UNHANDLED_INTERNAL_ERROR",
            ),
        })
        .into_response()
    }
}
