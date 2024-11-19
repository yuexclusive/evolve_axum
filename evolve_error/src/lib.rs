use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};

use axum_extra::typed_header::TypedHeaderRejection;
use jsonwebtoken::errors::Error as JwtError;
use redis::RedisError;
use std::fmt::Display;
pub use thiserror::Error;
use utoipa::ToSchema;

pub type AppResult<T> = std::result::Result<T, AppError>;

#[derive(Debug, Error)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
}

impl Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self))
    }
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    Wps(#[from] ErrorResp),

    #[error("internal error: {}",.msg)]
    Internal { msg: String },

    #[error("validate error: {}",.msg)]
    Validate { msg: String },

    #[error("hit: {}",.msg)]
    Hint { msg: String },

    #[error(transparent)]
    Auth(#[from] AuthError),

    #[error(transparent)]
    TypedHeaderRejection(#[from] TypedHeaderRejection),

    #[error(transparent)]
    Jwt(#[from] JwtError),

    #[error(transparent)]
    Redis(#[from] RedisError),

    #[error(transparent)]
    ChronoParse(#[from] chrono::ParseError),

    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),

    #[error(transparent)]
    Meilisearch(#[from] meilisearch_sdk::errors::Error),

    #[error(transparent)]
    SMTP(#[from] lettre::transport::smtp::Error),

    #[error(transparent)]
    Regex(#[from] fancy_regex::Error),

    #[error(transparent)]
    SerdeUrlEncodeError(#[from] serde_urlencoded::ser::Error),

    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),

    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),

    #[error(transparent)]
    InvalidHeaderValue(#[from] http::header::InvalidHeaderValue),

    #[error("other error occurred: {0}")]
    Other(#[from] anyhow::Error),
}

impl<T> From<AppError> for AppResult<T> {
    fn from(value: AppError) -> Self {
        Err(value)
    }
}

impl ErrorResp {
    fn new(code: usize, message: &str, details: Option<String>) -> Self {
        Self {
            code,
            msg: Some(message.to_string()),
            detail: details,
        }
    }
}

// How we want errors responses to be serialized
#[derive(Serialize, ToSchema, Deserialize, Debug)]
pub struct ErrorResp {
    pub code: usize,
    pub msg: Option<String>,
    pub detail: Option<String>,
}

impl std::fmt::Display for ErrorResp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", serde_json::to_string(&self).unwrap()))
    }
}

impl std::error::Error for ErrorResp {}

impl IntoResponse for ErrorResp {
    fn into_response(self) -> Response {
        axum::Json(self).into_response()
    }
}

// Tell axum how `AppError` should be converted into a response.
// This is also a convenient place to log errors.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, err_response) = match self {
            AppError::Wps(err_resp) => (StatusCode::INTERNAL_SERVER_ERROR, err_resp),

            AppError::Internal { msg } => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResp::new(500100000, &msg, None),
            ),

            AppError::Validate { msg } => (
                StatusCode::BAD_REQUEST,
                ErrorResp::new(400100000, &msg, None),
            ),

            AppError::Hint { msg } => (
                StatusCode::from_u16(452).unwrap(),
                ErrorResp::new(452100000, &msg, None),
            ),

            AppError::Auth(err) => match err {
                AuthError::MissingCredentials => (
                    StatusCode::BAD_REQUEST,
                    ErrorResp::new(400100001, "missing credentials", None),
                ),
                AuthError::WrongCredentials => (
                    StatusCode::UNAUTHORIZED,
                    ErrorResp::new(401100001, "wrong credentials", None),
                ),
                AuthError::TokenCreation => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ErrorResp::new(500100001, "token creation failed", None),
                ),
            },
            AppError::Jwt(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResp::new(400100002, &err.to_string(), None),
            ),
            AppError::TypedHeaderRejection(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResp::new(500100003, &err.to_string(), None),
            ),
            AppError::Redis(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResp::new(500100004, &err.to_string(), None),
            ),
            AppError::ChronoParse(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResp::new(500100005, &err.to_string(), None),
            ),
            AppError::Sqlx(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResp::new(500100006, &err.to_string(), None),
            ),
            AppError::Meilisearch(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResp::new(500100007, &err.to_string(), None),
            ),
            AppError::SMTP(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResp::new(500100008, &err.to_string(), None),
            ),
            AppError::Regex(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResp::new(500100009, &err.to_string(), None),
            ),
            _x => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResp::new(500999999, _x.to_string().as_str(), None),
            ),
        };

        (status, err_response).into_response()
    }
}
