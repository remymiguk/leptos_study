use leptos::SerializationError;
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum AppError {
    #[error("api fetch error: `{0}`")]
    FetchError(#[from] gloo_net::Error),
    #[error("api fetch error: `{0}`")]
    ReqWestError(#[from] reqwest::Error),
    #[error("serialization error: `{0}`")]
    SerializationError(#[from] SerializationError),
    #[error("database error: `{0}`")]
    Database(String),
    #[error("validation error: `{0}`")]
    Validation(String),
}

// impl IntoResponse for Error {
//     fn into_response(self) -> Response {
//         match self {
//             Error::Database(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
//             Error::Validation(e) => (StatusCode::BAD_REQUEST, e),
//         }
//         .into_response()
//     }
// }
