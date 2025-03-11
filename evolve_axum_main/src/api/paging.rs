use axum::{
    extract::{FromRequestParts, Query},
    http::request::Parts,
    response::{IntoResponse, Response},
    RequestPartsExt,
};
use evolve_error::AppError;
use serde::Deserialize;
use std::collections::HashMap;
use utoipa::IntoParams;

const MAX_PAGE_SIZE: i64 = 100;

#[derive(Default, IntoParams, Deserialize, Debug)]
pub struct Paging {
    #[param(example = 1)]
    pub page_index: i64,
    #[param(example = 10)]
    pub page_size: i64,
}

fn validate_page_index(page_index: &str) -> Result<i64, AppError> {
    let page_index = page_index
        .parse::<i64>()
        .map_err(|e| AppError::Validate { msg: e.to_string() })?;
    if page_index < 1 {
        return Err(AppError::Validate {
            msg: "page_index must be greater than 0".to_string(),
        });
    }
    Ok(page_index)
}

fn validate_page_size(page_size: &str) -> Result<i64, AppError> {
    let page_size = page_size
        .parse::<i64>()
        .map_err(|e| AppError::Validate { msg: e.to_string() })?;
    if page_size < 1 {
        return Err(AppError::Validate {
            msg: "page_size must be greater than 0".to_string(),
        });
    }
    if page_size > MAX_PAGE_SIZE {
        return Err(AppError::Validate {
            msg: format!("page_size must be less than or equal {}", MAX_PAGE_SIZE),
        });
    }
    Ok(page_size)
}

impl<S> FromRequestParts<S> for Paging
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let params: Query<HashMap<String, String>> =
            parts.extract().await.map_err(IntoResponse::into_response)?;

        let page_index = params.get("page_index");
        let page_size = params.get("page_size");

        match (page_index, page_size) {
            (Some(page_index), Some(page_size)) => Ok(Self {
                page_index: validate_page_index(page_index).map_err(|e| e.into_response())?,
                page_size: validate_page_size(page_size).map_err(|e| e.into_response())?,
            }),
            (Some(_), None) => Err(AppError::Validate {
                msg: format!("page_size is required"),
            }
            .into_response()),
            (None, _) => Err(AppError::Validate {
                msg: format!("page_index is required"),
            }
            .into_response()),
        }
    }
}
