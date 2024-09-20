use axum::{
    async_trait,
    extract::{FromRequestParts, Json},
    http::request::Parts,
    RequestPartsExt,
};
use axum_extra::{
    headers::{
        authorization::{Bearer, Credentials},
        Authorization,
    },
    TypedHeader,
};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use utoipa::{OpenApi, ToSchema};

use crate::model::user::{UserStatus, UserType};
use crate::service::user as user_service;
use crate::{env::ENV, error::AppError, error::ErrorResp};
use crate::{error::AuthError, model::user::CurrentUser};

struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

/** excute once only, for performence */
static KEYS: Lazy<Keys> = Lazy::new(|| Keys::new(ENV.jwt_secret.as_bytes()));

/** The Claims struct represents the data that will be encoded into the JWT. */
#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    /** The `sub` (subject) value is always the user ID. */
    pub sub: String,
    /** type of token */
    pub ty: AuthorizeType,
    /** The `iat` (issued at) value is the number of seconds elapsed since 00:00:00 UTC on January 1, 1970. */
    pub iat: usize,
    /** The `exp` (expiration time) value is the number of seconds elapsed since 00:00:00 UTC on January 1, 1970. */
    pub exp: usize,
}

#[derive(Serialize, ToSchema)]
pub struct AuthResp {
    access_token: String,
    expired_on: usize,
}

impl AuthResp {
    fn new(access_token: String, expired_on: usize) -> Self {
        Self {
            access_token,
            expired_on,
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Debug)]
pub enum AuthorizeType {
    APP,
    User,
    Enterprise,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct AuthReq {
    #[schema(default = "yu.exclusive@icloud.com")]
    id: String,
    #[schema(default = "a111111")]
    secret: String,
    #[schema(default = "User")]
    authorize_type: AuthorizeType,
}

#[utoipa::path(
    post,
    path = "/authorize",
    request_body = AuthReq,
    responses(
        (status = 200, description = "Successfully", body = AuthResp),
        (status = 400, description = "Missing credentials", body = ErrorResp),
        (status = 401, description = "Wrong credentials", body = ErrorResp),
        (status = 500, description = "Token creation error", body = ErrorResp),
    )
)]
pub async fn authorize(Json(req): Json<AuthReq>) -> Result<Json<AuthResp>, AppError> {
    // Check if the user sent the credentials
    if req.id.is_empty() || req.secret.is_empty() {
        return Err(AuthError::MissingCredentials.into());
    }

    let iat = chrono::Utc::now().timestamp() as usize;
    let exp = iat + ENV.token_valid_duraiton;

    match req.authorize_type {
        AuthorizeType::APP => {
            // Here you can check the user credentials from a database
            if req.id != "foo" || req.secret != "bar" {
                return Err(AuthError::WrongCredentials.into());
            }
        }
        AuthorizeType::Enterprise => {
            unimplemented!()
        }
        AuthorizeType::User => {
            user_service::authorize(&req.id, &req.secret).await?;
        }
    }

    let claims = Claims {
        sub: req.id,
        ty: req.authorize_type,
        iat,
        exp,
    };
    // create the authorization token
    // you can change algirithm with Header::new(jsonwebtoken::Algorithm::HS256)
    let token = jsonwebtoken::encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| AuthError::TokenCreation)?;

    // Send the authorized token
    Ok(Json(AuthResp::new(
        format!("{} {}", Bearer::SCHEME, token),
        exp,
    )))
}

#[derive(ToSchema, Serialize)]
pub struct UserInfoResp {
    data: CurrentUser,
}

#[utoipa::path(
    get,
    path = "/user_info",
    responses(
        (status = 200, description = "successfully", body = UserInfoResp),
        (status = 400, description = "bad request", body = ErrorResp),
        (status = 401, description = "unthorized", body = ErrorResp),
        (status = 500, description = "internal server error", body = ErrorResp)
    ),
    security(
        ("Authorization" = [])
    )
)]
pub async fn user_info(claims: Claims) -> Result<Json<UserInfoResp>, AppError> {
    match claims.ty {
        AuthorizeType::User => {
            let current_user = user_service::get_current_user(&claims.sub).await?;
            Ok(Json(UserInfoResp { data: current_user }))
        }
        _ => AppError::Validate {
            msg: "wrong token type".to_string(),
        }
        .into(),
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await?;

        // Decode the user data
        let token_data =
            jsonwebtoken::decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())?;

        Ok(token_data.claims)
    }
}

/// Item to do.

#[derive(OpenApi)]
#[openapi(
    paths(authorize, user_info),
    components(schemas(
        AuthReq,
        AuthResp,
        ErrorResp,
        CurrentUser,
        UserStatus,
        UserType,
        AuthorizeType,
        UserInfoResp
    ))
)]
pub struct JWTApi;
