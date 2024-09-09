use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

use axum_extra::typed_header::TypedHeaderRejection;
use jsonwebtoken::errors::Error as JwtError;
use redis::RedisError;
use std::fmt::Display;
pub use thiserror::Error;
use utoipa::ToSchema;

pub(crate) type AppResult<T> = std::result::Result<T, AppError>;

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
}

impl<T> From<AppError> for AppResult<T> {
    fn from(value: AppError) -> Self {
        Err(value)
    }
}

impl ErrorResp {
    fn new(code: usize, message: &str) -> Self {
        Self {
            code,
            message: message.to_string(),
        }
    }
}

// How we want errors responses to be serialized
#[derive(Serialize, ToSchema)]
pub struct ErrorResp {
    code: usize,
    message: String,
}

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
            AppError::Internal { msg } => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResp::new(500100000, &msg),
            ),

            AppError::Validate { msg } => {
                (StatusCode::BAD_REQUEST, ErrorResp::new(400100000, &msg))
            }

            AppError::Hint { msg } => (
                StatusCode::from_u16(452).unwrap(),
                ErrorResp::new(452100000, &msg),
            ),

            AppError::Auth(err) => match err {
                AuthError::MissingCredentials => (
                    StatusCode::BAD_REQUEST,
                    ErrorResp::new(400100001, "missing credentials"),
                ),
                AuthError::WrongCredentials => (
                    StatusCode::UNAUTHORIZED,
                    ErrorResp::new(401100001, "wrong credentials"),
                ),
                AuthError::TokenCreation => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ErrorResp::new(500100001, "token creation failed"),
                ),
            },
            AppError::Jwt(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResp::new(400100002, &err.to_string()),
            ),
            AppError::TypedHeaderRejection(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResp::new(500100003, &err.to_string()),
            ),
            AppError::Redis(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResp::new(500100004, &err.to_string()),
            ),
            AppError::ChronoParse(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResp::new(500100005, &err.to_string()),
            ),
            AppError::Sqlx(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResp::new(500100006, &err.to_string()),
            ),
            AppError::Meilisearch(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResp::new(500100007, &err.to_string()),
            ),
            AppError::SMTP(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResp::new(500100008, &err.to_string()),
            ),
            AppError::Regex(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResp::new(500100009, &err.to_string()),
            ),
        };

        (status, err_response).into_response()
    }
}
