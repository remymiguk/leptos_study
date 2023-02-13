use std::collections::HashMap;

use leptos::SerializationError;
use serde::{Deserialize, Serialize};
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum AppError {
    #[error("api fetch error: `{0}`")]
    FetchError(#[from] gloo_net::Error),
    #[error("api fetch error: `{0}`")]
    ReqWestError(#[from] reqwest::Error),
    #[error("serialization error: `{0}`")]
    SerializationError(#[from] SerializationError),
    #[error("parse json error: `{0}`")]
    ParseJson(#[from] serde_json::Error),
    #[error("database error: `{0}`")]
    Database(String),
    #[error("validation error: `{0}`")]
    Validation(String),

    #[error("request error: `{0}`")]
    Request(String),
    /// 302, 303, 307
    #[error("redirect to `{0}`")]
    Redirect(String),
    /// 401
    #[error("unauthorized")]
    Unauthorized,
    /// 403
    #[error("forbidden")]
    Forbidden,
    /// 404
    #[error("not found")]
    NotFound,
    /// 422
    #[error("unprocessable entity: {0:?}")]
    UnprocessableEntity(ErrorInfo),
    /// 500
    #[error("internal server error")]
    InternalServerError,
    #[error("http request error: `{0}`")]
    RequestError(String),
}

/// Conduit api error info for Unprocessable Entity error
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInfo {
    pub errors: HashMap<String, Vec<String>>,
}
