use crate::error::AppResult;
use crate::model::user as user_model;
use crate::redis as redis_util;

fn email_code_key(email: &str, from: &user_model::SendEmailCodeFrom) -> String {
    format!("{email}_mail_{:?}", from)
}

pub async fn get_email_code(
    email: &str,
    from: &user_model::SendEmailCodeFrom,
) -> AppResult<Option<String>> {
    let res = redis_util::get::<_, Option<String>>(email_code_key(email, from)).await?;

    Ok(res)
}

pub async fn set_email_code(
    email: &str,
    from: &user_model::SendEmailCodeFrom,
    code: impl Into<String>,
    expired_seconds: u64,
) -> AppResult<()> {
    redis_util::set_ex(email_code_key(email, from), code.into(), expired_seconds).await?;
    Ok(())
}

pub async fn exist_email_code(
    email: &str,
    from: &user_model::SendEmailCodeFrom,
) -> AppResult<bool> {
    let res = redis_util::exists(email_code_key(email, from)).await?;
    Ok(res)
}
