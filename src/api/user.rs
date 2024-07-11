use crate::service::user as user_service;
// use actix_web::web::{Json, Path, Query};
// use actix_web::{delete, get, post, put, HttpRequest, Responder, Result};
use serde::Deserialize;
// use util_response::{data, msg, prelude::*};
use crate::api::auth::Claims;
use crate::error::{AppError, ErrorResponse};
use crate::model::user::{
    ChangePasswordReq, RegisterReq, SearchedUser, SendEmailCodeReq, User, UserDeleteReq,
    UserUpdateReq,
};
use crate::response::{data, data_with_total, msg};
use crate::response::{DataResponse, MsgResponse, Pagination};
use axum::{
    extract::{Path, Query},
    Json,
};
use utoipa::{IntoParams, OpenApi};

#[utoipa::path(
    post,
    request_body = SendEmailCodeReq,
    path = "/send_email_code",
    responses(
        (status = 200, description = "successfully", body = DataResponse<u64>),
        (status = 400, description = "bad request", body = ErrorResponse),
        (status = 500, description = "internal server error", body = ErrorResponse)
    )
)]
pub async fn send_email_code(
    Json(req): Json<SendEmailCodeReq>,
) -> Result<Json<DataResponse<u64>>, AppError> {
    let res = user_service::send_email_code(&req.email, &req.from).await?;
    Ok(Json(data(res)))
}

#[utoipa::path(
    put,
    request_body = ChangePasswordReq,
    path = "/change_pwd",
    responses(
        (status = 200, description = "successfully", body = MsgResponse),
        (status = 400, description = "bad request", body = ErrorResponse),
        (status = 500, description = "internal server error", body = ErrorResponse)
    )
)]
pub async fn change_pwd(Json(req): Json<ChangePasswordReq>) -> Result<Json<MsgResponse>, AppError> {
    let _ = user_service::change_pwd(&req.email, &req.code, &req.pwd).await?;
    Ok(Json(msg("ok")))
}

#[utoipa::path(
    post,
    request_body = RegisterReq,
    path = "/register",
    responses(
        (status = 200, description = "successfully", body = MsgResponse),
        (status = 400, description = "bad request", body = ErrorResponse),
        (status = 500, description = "internal server error", body = ErrorResponse)
    )
)]
pub async fn register(req: Json<RegisterReq>) -> Result<Json<MsgResponse>, AppError> {
    user_service::register(
        &req.email,
        &req.code,
        &req.pwd,
        req.name.as_deref(),
        req.mobile.as_deref(),
    )
    .await?;
    Ok(Json(msg("ok")))
}

#[utoipa::path(
    get,
    path = "/validate_exist_email/{email}",
    params(
        ("email", description = "email")
    ),
    responses(
        (status = 200, description = "successfully", body = MsgResponse),
        (status = 400, description = "bad request", body = ErrorResponse),
        (status = 500, description = "internal server error", body = ErrorResponse)
    )
)]
pub async fn validate_exist_email(
    Path(email): Path<String>,
) -> Result<Json<MsgResponse>, AppError> {
    user_service::validate_exist_email(&email).await?;

    Ok(Json(msg("ok")))
}

#[utoipa::path(
    get,
    path = "/validate_not_exist_email/{email}",
    params(
        ("email", description = "email")
    ),
    responses(
        (status = 200, description = "successfully", body = MsgResponse),
        (status = 400, description = "bad request", body = ErrorResponse),
        (status = 500, description = "internal server error", body = ErrorResponse)
    )
)]
pub async fn validate_not_exist_email(
    Path(email): Path<String>,
) -> Result<Json<MsgResponse>, AppError> {
    user_service::validate_not_exist_email(&email).await?;

    Ok(Json(msg("ok")))
}

#[derive(IntoParams, Deserialize)]
pub struct SearchReq {
    key_word: Option<String>,
}
#[utoipa::path(
    get,
    path = "/search",
    params(
        SearchReq, Pagination
    ),
    responses(
        (status = 200, description = "successfully", body = DataResponse<Vec<SearchedUser>>),
        (status = 400, description = "bad request", body = ErrorResponse),
        (status = 401, description = "unthorized", body = ErrorResponse),
        (status = 500, description = "internal server error", body = ErrorResponse)
    ),
    security(
        ("token" = [])
    )
)]
pub async fn search(
    Query(req): Query<SearchReq>,
    Query(page): Query<Pagination>,
    _claims: Claims,
) -> Result<Json<DataResponse<Vec<SearchedUser>>>, AppError> {
    let (data, total) = user_service::search(&req.key_word.unwrap_or_default(), &page).await?;
    Ok(Json(data_with_total(data, total)))
}

#[utoipa::path(
    get,
    path = "/{id}",
    params(
        ("id", description = "user id")
    ),
    responses(
        (status = 200, description = "successfully", body = DataResponse<User>),
        (status = 400, description = "bad request", body = ErrorResponse),
        (status = 500, description = "internal server error", body = ErrorResponse)
    ),
    security(
        ("token" = [])
    )
)]
pub async fn get(Path(id): Path<i64>) -> Result<Json<DataResponse<User>>, AppError> {
    let res = user_service::get(id).await?;
    Ok(Json(data(res)))
}

#[utoipa::path(
    put,
    request_body = UserUpdateReq,
    path = "/update",
    responses(
        (status = 200, description = "successfully", body = DataResponse<User>),
        (status = 400, description = "bad request", body = ErrorResponse),
        (status = 401, description = "unthorized", body = ErrorResponse),
        (status = 500, description = "internal server error", body = ErrorResponse)
    ),
    security(
        ("token" = [])
    )
)]
pub async fn update(Json(req): Json<UserUpdateReq>) -> Result<Json<DataResponse<User>>, AppError> {
    let res = user_service::update(req.id, req.mobile.as_deref(), req.name.as_deref()).await?;
    Ok(Json(data(res)))
}

#[utoipa::path(
    delete,
    request_body = UserDeleteReq,
    path = "/delete",
    responses(
        (status = 200, description = "successfully", body = MsgResponse),
        (status = 400, description = "bad request", body = ErrorResponse),
        (status = 401, description = "unthorized", body = ErrorResponse),
        (status = 500, description = "internal server error", body = ErrorResponse)
    ),
    security(
        ("token" = [])
    )
)]
pub async fn delete(Json(req): Json<UserDeleteReq>) -> Result<Json<MsgResponse>, AppError> {
    let _ = user_service::delete(&req.ids).await?;
    Ok(Json(msg("ok")))
}

#[derive(OpenApi)]
#[openapi(
    paths(
        send_email_code,
        change_pwd,
        register,
        validate_exist_email,
        validate_not_exist_email,
        search,
        get,
        update,
        delete
    ),
    components(schemas(ChangePasswordReq,RegisterReq,UserUpdateReq,SendEmailCodeReq,ErrorResponse,MsgResponse,DataResponse<u64>,DataResponse<Vec<SearchedUser>>,DataResponse<User>))
)]
pub struct UserApi;
