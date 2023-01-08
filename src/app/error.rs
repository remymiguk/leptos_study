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
