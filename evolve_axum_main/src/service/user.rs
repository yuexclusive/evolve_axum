use std::collections::HashSet;

use evolve_axum_dao::{
    meilisearch::{self as meilisearch_dao, user as meilisearch_user_dao},
    model::user as user_model,
    pg::user as pg_user_dao,
    redis::user as redis_user_dao,
};

use futures::TryFutureExt;
use rand::Rng;

use serde::{Deserialize, Serialize};

use evolve_error::{AppError, AppResult};

mod private {
    use base64::{engine::general_purpose, Engine as _};

    const EMAIL_VALIDATE_REGEX: &str = r#"\w[-\w.+]*@([A-Za-z0-9][-A-Za-z0-9]+\.)+[A-Za-z]{2,14}"#;
    const PWD_VALIDATE_REGEX: &str = r#"(?=.*[a-z])(?=.*[0-9])[a-zA-Z0-9]{6,18}"#;
    const MOBILE_VALIDATE_REGEX: &str = r#"0?(13|14|15|17|18|19)[0-9]{9}"#;

    use std::cmp::Ordering;

    use super::{
        meilisearch_dao, pg_user_dao, redis_user_dao, user_model, AppResult, Deserialize, Serialize,
    };
    use evolve_error::AppError;
    use fancy_regex::Regex;
    use sha2::Digest;
    use uuid::Uuid;

    #[derive(Debug, Serialize, Deserialize)]
    struct Claims {
        aud: String, // Optional. Audience
        exp: u64, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
        iat: u64, // Optional. Issued at (as UTC timestamp)
                  // sub: Option<String>, // Optional. Subject (whom token refers to)
                  // iss: String, // Optional. Issuer
                  // nbf: usize, // Optional. Not Before (as UTC timestamp)
    }

    pub(super) fn hash_password(password: impl AsRef<str>, salt: impl AsRef<str>) -> String {
        let mut hasher = sha2::Sha512::new();
        hasher.update(password.as_ref());
        hasher.update(b"$");
        hasher.update(salt.as_ref());
        let encoded: String = general_purpose::STANDARD.encode(&hasher.finalize());
        encoded
    }

    pub(super) fn salt() -> String {
        Uuid::new_v4().to_string()
    }

    pub(super) fn check_pwd(
        pwd: impl AsRef<str>,
        salt: impl AsRef<str>,
        pwd_hashed: Option<impl AsRef<str>>,
    ) -> AppResult<()> {
        match pwd_hashed {
            Some(v) => {
                let pwd = hash_password(pwd, salt);
                match pwd.as_str().cmp(v.as_ref()) {
                    Ordering::Equal => Ok(()),
                    _ => AppError::Validate {
                        msg: "wrong password".to_string(),
                    }
                    .into(),
                }
            }
            None => AppError::Validate {
                msg: "password has not been initialized".to_string(),
            }
            .into(),
        }
    }

    pub(super) fn validate_email(email: &str) -> AppResult<()> {
        match email.is_empty() {
            true => AppError::Validate {
                msg: "please type in email".to_string(),
            }
            .into(),
            _ => {
                let reg = Regex::new(EMAIL_VALIDATE_REGEX)?;
                match reg.is_match(email)? {
                    false => AppError::Validate {
                        msg: "invald email".to_string(),
                    }
                    .into(),
                    _ => Ok(()),
                }
            }
        }
    }

    pub(super) fn validate_pwd(pwd: &str) -> AppResult<()> {
        match pwd.is_empty() {
            true => AppError::Validate {
                msg: "please type in password".to_string(),
            }
            .into(),
            _ => {
                let reg = Regex::new(PWD_VALIDATE_REGEX)?; //6位字母+数字,字母开头
                match reg.is_match(pwd)? {
                    false => AppError::Validate {
                        msg: "invalid passowrd: length>=6, a-z and 0-9 is demanded".to_string(),
                    }
                    .into(),
                    _ => Ok(()),
                }
            }
        }
    }

    pub(super) fn validate_mobile(mobile: &str) -> AppResult<()> {
        let reg = Regex::new(MOBILE_VALIDATE_REGEX)?;
        match reg.is_match(mobile)? {
            false => AppError::Validate {
                msg: "invalid mobile".to_string(),
            }
            .into(),
            _ => Ok(()),
        }
    }

    pub(super) async fn validate_email_code(
        email: &str,
        from: &user_model::SendEmailCodeFrom,
        code: &str,
    ) -> AppResult<()> {
        let email_code: Option<String> = redis_user_dao::get_email_code(email, from).await?;

        match email_code.is_none() || email_code.unwrap() != code {
            true => AppError::Validate {
                msg: format!("invalid code: {}", code),
            }
            .into(),
            _ => Ok(()),
        }
    }

    pub(super) async fn validate_exist_email(email: &str) -> AppResult<pg_user_dao::User> {
        validate_email(email)?;
        let res = pg_user_dao::get_by_email(email).await?;
        match res.deleted_at {
            Some(_) => AppError::Validate {
                msg: "email has already been deleted".to_string(),
            }
            .into(),
            _ => Ok(res),
        }
    }

    pub(super) async fn validate_not_exist_email(email: &str) -> AppResult<()> {
        validate_email(email)?;
        match pg_user_dao::get_by_email(email).await {
            Ok(_) => AppError::Validate {
                msg: format!("email {} already exist", email),
            }
            .into(),
            _ => Ok(()),
        }
    }

    pub(super) async fn update_search<T>(data: T) -> AppResult<()>
    where
        T: Into<user_model::User>,
    {
        meilisearch_dao::update(meilisearch_dao::USER_LIST_INDEX, &[data.into()], Some("id"))
            .await?;
        Ok(())
    }
}

pub async fn search(
    key_word: &str,
    page: &evolve_axum_dao::Paging,
) -> AppResult<(Vec<user_model::SearchedUser>, usize)> {
    meilisearch_user_dao::search(key_word, page).await
}

pub async fn get(id: i64) -> AppResult<user_model::User> {
    let res = pg_user_dao::get(id).await?;
    let user = res.ok_or(AppError::NotFound {
        msg: format!("user with id {} cannot be found", id),
    })?;
    Ok(user.into())
}

pub async fn get_all() -> AppResult<Vec<user_model::User>> {
    let res = pg_user_dao::get_all()
        .await?
        .into_iter()
        .map(|x| x.into())
        .collect();
    Ok(res)
}

pub async fn register(
    email: &str,
    code: &str,
    pwd: &str,
    name: Option<&str>,
    mobile: Option<&str>,
) -> AppResult<user_model::User> {
    private::validate_not_exist_email(email).await?;
    private::validate_pwd(pwd)?;
    private::validate_email_code(email, &user_model::SendEmailCodeFrom::Register, code).await?;
    if let Some(x) = mobile {
        private::validate_mobile(x)?;
    }
    let salt = private::salt();
    let pwd = private::hash_password(pwd, &salt);

    let current_user = pg_user_dao::insert(email, &salt, &pwd, name, mobile).await?;
    private::update_search(current_user.clone()).await?;
    Ok(current_user.into())
}

pub async fn authorize(email: &str, pwd: &str) -> AppResult<()> {
    let mut user = private::validate_exist_email(email).await?;

    private::check_pwd(pwd, &user.salt, user.pwd.as_deref())?;
    let now = chrono::Utc::now();
    user.laston = Some(now);
    let user_copy = user.clone();

    tokio::try_join!(
        pg_user_dao::update_laston(user.id, &now).map_err(|err| err.into()),
        private::update_search(user_copy),
    )?;

    return Ok(());
}

pub async fn validate_not_exist_email(email: &str) -> AppResult<()> {
    private::validate_not_exist_email(email).await
}

pub async fn validate_exist_email(email: &str) -> AppResult<user_model::User> {
    Ok(private::validate_exist_email(email).await?.into())
}

pub async fn delete(ids: &[i64]) -> AppResult<Vec<i64>> {
    let (pg_del_res, _) = tokio::try_join!(
        pg_user_dao::delete(ids).map_err(|err| err.into()),
        meilisearch_dao::delete(meilisearch_dao::USER_LIST_INDEX, ids)
    )?;
    let in_hs = ids.iter().map(|x| *x).collect::<HashSet<i64>>();
    let out_hs = pg_del_res.iter().map(|x| x.id).collect::<HashSet<i64>>();
    let diff = in_hs.difference(&out_hs).map(|x| *x).collect::<Vec<i64>>();
    if !diff.is_empty() {
        return AppError::NotFound {
            msg: format!("some users with ids {:?} cannot be deleted", diff),
        }
        .into();
    }
    Ok(out_hs.iter().map(|x| *x).collect())
}

pub async fn change_pwd(email: &str, code: &str, new_pwd: &str) -> AppResult<u64> {
    let user = validate_exist_email(email).await?;
    private::validate_email_code(email, &user_model::SendEmailCodeFrom::ChangePwd, code).await?;
    private::validate_pwd(new_pwd)?;

    let salt = private::salt();
    let pwd = private::hash_password(new_pwd, &salt);
    let res = pg_user_dao::update_pwd(user.id, &salt, &pwd).await?;

    Ok(res)
}

pub async fn send_email_code(email: &str, from: &user_model::SendEmailCodeFrom) -> AppResult<u64> {
    match from {
        user_model::SendEmailCodeFrom::Register => validate_not_exist_email(email).await?,
        user_model::SendEmailCodeFrom::ChangePwd => {
            private::validate_exist_email(email).await?;
            ()
        }
    }

    if redis_user_dao::exist_email_code(email, from).await? {
        return AppError::Hint{msg:"the validation code has already send to your mail box, please check or resend after a few minutes".to_string()}.into();
    }

    let code = rand::thread_rng().gen_range(100000..999999);

    let expired_seconds = 120;

    let body = format!("the validation code is: {}", code);

    let (cache_code_res, send_code_res) = tokio::join!(
        redis_user_dao::set_email_code(email, from, code.to_string(), expired_seconds),
        evolve_mailer::send(email, "validation code", &body)
    );
    let _ = cache_code_res?;
    let _ = send_code_res?;
    Ok(expired_seconds)
}

pub async fn update(
    id: i64,
    mobile: Option<&str>,
    name: Option<&str>,
) -> AppResult<user_model::User> {
    if let Some(mobile) = mobile {
        private::validate_mobile(mobile)?;
    }
    let res = pg_user_dao::update(id, name, mobile).await?;
    private::update_search(res.clone()).await?;
    Ok(res.into())
}

pub async fn get_current_user(email: &str) -> AppResult<user_model::CurrentUser> {
    let user = pg_user_dao::get_by_email(email).await?;
    let res = user_model::CurrentUser {
        id: user.id,
        r#type: user.r#type,
        email: user.email,
        status: user.status,
        mobile: user.mobile,
        name: user.name,
        created_at: user.created_at.timestamp(),
        updated_at: user.updated_at.map(|x| x.timestamp()),
        laston: user.laston.map(|x| x.timestamp()),
    };
    Ok(res)
}

pub async fn load_search() -> AppResult<()> {
    let data = get_all().await?;
    let documents = data
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<user_model::User>>();

    meilisearch_dao::reload(meilisearch_dao::USER_LIST_INDEX, &documents, Some("id")).await?;
    Ok(())
}
