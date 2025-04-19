use axum::{http::StatusCode, response::IntoResponse};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    BuyProductFailed,
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        println!("->> Error: {:?}", self);

        match self {
            Error::BuyProductFailed => (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_SERVER_ERROR"),
        }
        .into_response()
    }
}
