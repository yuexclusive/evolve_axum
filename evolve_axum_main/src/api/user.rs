use super::msg;
use super::MsgResp;
use super::Pagination;
use crate::api::auth::Claims;
use crate::service::user as user_service;
use axum::{
    extract::{Path, Query},
    Json,
};
use evolve_axum_dao::model::user::SearchedUser;
use evolve_axum_dao::model::user::SendEmailCodeFrom;
use evolve_axum_dao::model::user::User;
use evolve_axum_dao::model::user::UserFormatter;
use evolve_error::{AppError, ErrorResp};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, OpenApi, ToSchema};

#[derive(ToSchema, Deserialize)]
pub struct SendEmailCodeReq {
    #[schema(default = "yu.exclusive@icloud.com")]
    pub email: String,
    #[schema(default = "ChangePwd")]
    pub from: SendEmailCodeFrom,
}

#[derive(ToSchema, Deserialize)]
pub struct ChangePasswordReq {
    /// email
    pub email: String,
    /// validate code
    pub code: String,
    /// password
    pub pwd: String,
}

#[derive(ToSchema, Deserialize)]
pub struct UserUpdateReq {
    pub id: i64,
    pub name: Option<String>,
    pub mobile: Option<String>,
}

#[derive(ToSchema, Deserialize)]
pub struct RegisterReq {
    pub email: String,
    pub pwd: String,
    pub code: String,
    pub name: Option<String>,
    pub mobile: Option<String>,
}

#[derive(ToSchema, Deserialize)]
pub struct UserDeleteReq {
    pub ids: Vec<i64>,
}

#[derive(Serialize, ToSchema)]
pub struct SendEmailCodeResp {
    data: u64,
}

#[utoipa::path(
    post,
    request_body = SendEmailCodeReq,
    path = "/send_email_code",
    responses(
        (status = 200, description = "successfully", body = SendEmailCodeResp),
        (status = 400, description = "bad request", body = ErrorResp),
        (status = 500, description = "internal server error", body = ErrorResp)
    )
)]
pub async fn send_email_code(
    Json(req): Json<SendEmailCodeReq>,
) -> Result<Json<SendEmailCodeResp>, AppError> {
    let res = user_service::send_email_code(&req.email, &req.from).await?;
    Ok(Json(SendEmailCodeResp { data: res }))
}

#[utoipa::path(
    put,
    request_body = ChangePasswordReq,
    path = "/change_pwd",
    responses(
        (status = 200, description = "successfully", body = MsgResp),
        (status = 400, description = "bad request", body = ErrorResp),
        (status = 500, description = "internal server error", body = ErrorResp)
    )
)]
pub async fn change_pwd(Json(req): Json<ChangePasswordReq>) -> Result<Json<MsgResp>, AppError> {
    let _ = user_service::change_pwd(&req.email, &req.code, &req.pwd).await?;
    Ok(Json(msg("ok")))
}

#[utoipa::path(
    post,
    request_body = RegisterReq,
    path = "/register",
    responses(
        (status = 200, description = "successfully", body = MsgResp),
        (status = 400, description = "bad request", body = ErrorResp),
        (status = 500, description = "internal server error", body = ErrorResp)
    )
)]
pub async fn register(req: Json<RegisterReq>) -> Result<Json<MsgResp>, AppError> {
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
        ("email", description = "email", example = "yu.exclusive@icloud.com")
    ),
    responses(
        (status = 200, description = "successfully", body = MsgResp),
        (status = 400, description = "bad request", body = ErrorResp),
        (status = 500, description = "internal server error", body = ErrorResp)
    )
)]
pub async fn validate_exist_email(
    Path((_v, email)): Path<(String, String)>,
) -> Result<Json<MsgResp>, AppError> {
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
        (status = 200, description = "successfully", body = MsgResp),
        (status = 400, description = "bad request", body = ErrorResp),
        (status = 500, description = "internal server error", body = ErrorResp)
    )
)]
pub async fn validate_not_exist_email(
    Path((_v, email)): Path<(String, String)>,
) -> Result<Json<MsgResp>, AppError> {
    user_service::validate_not_exist_email(&email).await?;

    Ok(Json(msg("ok")))
}

#[derive(Serialize, ToSchema)]
pub struct UserSearchResp {
    data: Vec<SearchedUser>,
    total: usize,
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
        (status = 200, description = "successfully", body = UserSearchResp),
        (status = 400, description = "bad request", body = ErrorResp),
        (status = 401, description = "unthorized", body = ErrorResp),
        (status = 500, description = "internal server error", body = ErrorResp)
    ),
    security(
        ("Authorization" = [])
    )
)]
pub async fn search(
    Query(req): Query<SearchReq>,
    Query(page): Query<Pagination>,
    _claims: Claims,
) -> Result<Json<UserSearchResp>, AppError> {
    let (data, total) = user_service::search(
        &req.key_word.unwrap_or_default(),
        &evolve_axum_dao::Pagination {
            index: page.index,
            size: page.size,
        },
    )
    .await?;
    Ok(Json(UserSearchResp { data, total }))
}

#[derive(Serialize, ToSchema)]
pub struct UserGetResp {
    data: User,
}
#[utoipa::path(
    get,
    path = "/{id}",
    params(
        ("id", description = "user id")
    ),
    responses(
        (status = 200, description = "successfully", body = UserGetResp),
        (status = 400, description = "bad request", body = ErrorResp),
        (status = 500, description = "internal server error", body = ErrorResp)
    ),
    security(
        ("Authorization" = [])
    )
)]
pub async fn get(Path(id): Path<i64>) -> Result<Json<UserGetResp>, AppError> {
    let res = user_service::get(id).await?;
    Ok(Json(UserGetResp { data: res }))
}

#[derive(Serialize, ToSchema)]
pub struct UserUpdateResp {
    data: User,
}
#[utoipa::path(
    put,
    request_body = UserUpdateReq,
    path = "/update",
    responses(
        (status = 200, description = "successfully", body = UserUpdateResp),
        (status = 400, description = "bad request", body = ErrorResp),
        (status = 401, description = "unthorized", body = ErrorResp),
        (status = 500, description = "internal server error", body = ErrorResp)
    ),
    security(
        ("Authorization" = [])
    )
)]
pub async fn update(Json(req): Json<UserUpdateReq>) -> Result<Json<UserUpdateResp>, AppError> {
    let res = user_service::update(req.id, req.mobile.as_deref(), req.name.as_deref()).await?;
    Ok(Json(UserUpdateResp { data: res }))
}

#[utoipa::path(
    delete,
    request_body = UserDeleteReq,
    path = "/delete",
    responses(
        (status = 200, description = "successfully", body = MsgResp),
        (status = 400, description = "bad request", body = ErrorResp),
        (status = 401, description = "unthorized", body = ErrorResp),
        (status = 500, description = "internal server error", body = ErrorResp)
    ),
    security(
        ("Authorization" = [])
    )
)]
pub async fn delete(Json(req): Json<UserDeleteReq>) -> Result<Json<MsgResp>, AppError> {
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
    components(schemas(
        ChangePasswordReq,
        RegisterReq,
        UserUpdateReq,
        SendEmailCodeReq,
        SendEmailCodeFrom,
        UserDeleteReq,
        ErrorResp,
        MsgResp,
        SendEmailCodeResp,
        UserSearchResp,
        UserGetResp,
        UserUpdateResp,
        SearchedUser,
        User,
        UserFormatter
    ))
)]
pub struct UserApi;
