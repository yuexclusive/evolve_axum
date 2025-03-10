use crate::model::user::{self as user_model, UserStatus, UserType};
use crate::Paging;
use chrono::{DateTime, Utc};
use evolve_datetime::FormatDateTime;
use evolve_error::{AppError, AppResult};

use evolve_util::postgres_util::{conn, SqlResult};

#[derive(Debug, Clone)]
pub struct User {
    pub id: i64,
    pub r#type: UserType,   // 1. normal 2. admin 3.super admin
    pub status: UserStatus, // 1. available 2. disabled
    pub email: String,
    pub name: Option<String>,
    pub salt: String,
    pub pwd: Option<String>,
    pub mobile: Option<String>,
    pub laston: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl From<User> for user_model::User {
    fn from(x: User) -> Self {
        user_model::User {
            id: x.id,
            r#type: x.r#type,
            email: x.email,
            status: x.status,
            name: x.name,
            mobile: x.mobile,
            laston: x.laston.map(|x| x.to_default()),
            created_at: x.created_at.to_default(),
            updated_at: x.updated_at.map(|x| x.to_default()),
        }
    }
}

#[allow(unused)]
pub async fn count() -> SqlResult<usize> {
    let res = sqlx::query!(
        r#"
select
    count(1) 
from
"user" u
where u.deleted_at is null
"#
    )
    .fetch_one(conn().await)
    .await?;
    Ok(res.count.unwrap() as usize)
}

#[allow(unused)]
pub async fn query(p: &Paging) -> SqlResult<Vec<User>> {
    sqlx::query_as!(
        User,
        r#"
select
     id,
     "type" as "type!: UserType",
     email,
     status as "status!: UserStatus",
     "name",
     salt,
     pwd,
     mobile,
     laston,
     created_at,
     updated_at,
     deleted_at
from "user" u
where u.deleted_at is null
order by u.created_at desc
limit $1 offset $2
"#,
        p.take(),
        p.skip(),
    )
    .fetch_all(conn().await)
    .await
}

pub async fn get_all() -> SqlResult<Vec<User>> {
    sqlx::query_as!(
        User,
        r#"
select
     id,
     "type" as "type!: UserType",
     email,
     status as "status!: UserStatus",
     "name",
     salt,
     pwd,
     mobile,
     laston,
     created_at,
     updated_at,
     deleted_at
from "user" u
where u.deleted_at is null
order by u.created_at desc
"#,
    )
    .fetch_all(conn().await)
    .await
}

pub async fn get(id: i64) -> SqlResult<Option<User>> {
    sqlx::query_as!(
        User,
        r#"
select
    id,
    "type" as "type!: UserType",
    email,
    status as "status!: UserStatus",
    "name",
    salt,
    pwd,
    mobile,
    laston,
    created_at,
    updated_at,
    deleted_at
from "user" 
where id = $1 and deleted_at is null
            "#,
        id,
    )
    .fetch_optional(conn().await)
    .await
}

pub async fn get_by_email(email: &str) -> AppResult<User> {
    let res = sqlx::query_as!(
        User,
        r#"
select
    id,
    "type" as "type!: UserType",
    email,
    status as "status!: UserStatus",
    "name",
    salt,
    pwd,
    mobile,
    laston,
    created_at,
    updated_at,
    deleted_at
from "user" 
where email = $1
            "#,
        email,
    )
    .fetch_one(conn().await)
    .await
    .map_err(|err| match err {
        // sqlx::Error::RowNotFound => validate_error!(format!("email: {} is not exist", email)),
        sqlx::Error::RowNotFound => AppError::Validate {
            msg: format!("email: {} is not exist", email),
        },
        _ => err.into(),
    })?;

    Ok(res)
}

pub async fn insert(
    email: &str,
    salt: &str,
    pwd: &str,
    name: Option<&str>,
    mobile: Option<&str>,
) -> SqlResult<User> {
    let created_at = chrono::Local::now();
    let res = sqlx::query_as!(
        User,
        r#"
insert into "user" (type,email,pwd,salt,name,mobile,created_at) values ($1,$2,$3,$4,$5,$6,$7) 
RETURNING 

id,
"type" as "type!: UserType",
email,
status as "status!: UserStatus",
"name",
salt,
pwd,
mobile,
laston,
created_at,
updated_at,
deleted_at           
            "#,
        UserType::Normal as UserType,
        email,
        pwd,
        salt,
        name,
        mobile,
        created_at,
    )
    .fetch_one(conn().await)
    .await?;

    Ok(res)
}

pub struct UserDeleteResult {
    pub id: i64,
}

pub async fn delete(ids: &[i64]) -> SqlResult<Vec<UserDeleteResult>> {
    let deleted_at = chrono::Local::now();
    let res = sqlx::query_as!(UserDeleteResult,
        r#"update "user" set deleted_at = $1 where id = ANY($2) and deleted_at is null RETURNING id"#,
        deleted_at,
        ids,
    )
    .fetch_all(conn().await)
    .await?;

    Ok(res)
}

pub async fn update(id: i64, name: Option<&str>, mobile: Option<&str>) -> SqlResult<User> {
    let updated_at = chrono::Local::now();
    sqlx::query_as!(
        User,
        r#"update "user" set name = $1, mobile = $2, updated_at=$3 where id = $4 RETURNING
id,
"type" as "type!: UserType",
email,
status as "status!: UserStatus",
"name",
salt,
pwd,
mobile,
laston,
created_at,
updated_at,
deleted_at
            "#,
        name,
        mobile,
        updated_at,
        id,
    )
    .fetch_one(conn().await)
    .await
}

pub async fn update_pwd(id: i64, salt: &str, pwd: &str) -> SqlResult<u64> {
    let updated_at = chrono::Local::now();
    let res = sqlx::query!(
        r#"update "user" set salt = $1, pwd = $2, updated_at=$3 where id = $4"#,
        salt,
        pwd,
        updated_at,
        id,
    )
    .execute(conn().await)
    .await?;

    Ok(res.rows_affected())
}

pub async fn update_laston(id: i64, laston: &DateTime<Utc>) -> SqlResult<u64> {
    let res = sqlx::query!(r#"update "user" set laston = $1 where id = $2"#, laston, id)
        .execute(conn().await)
        .await?;

    Ok(res.rows_affected())
}
