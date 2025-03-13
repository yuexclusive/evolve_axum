use chrono::{FixedOffset, TimeZone};
pub use entity::user::Model as User;
use evolve_datetime::FormatDateTime;
use evolve_error::AppResult;
use evolve_util::seaorm_util::{self, Paging};
use sea_orm::{
    ActiveModelTrait,
    ActiveValue::{self, Set},
    ColumnTrait, Condition, Database, DeleteResult, EntityTrait, Order, PaginatorTrait,
    QueryFilter, QueryOrder, QuerySelect,
};

use crate::{
    model::user as user_model,
    pg_seaorm::entity::sea_orm_active_enums::{Userstatus, Usertype},
};

use super::entity;

pub async fn count() -> AppResult<u64> {
    let conn = seaorm_util::conn().await;

    let res = entity::user::Entity::find()
        .filter(entity::user::Column::DeletedAt.is_null())
        .count(conn)
        .await?;

    Ok(res)
}

pub async fn query(p: &Paging) -> AppResult<Vec<User>> {
    let conn = seaorm_util::conn().await;

    let res = entity::user::Entity::find()
        .filter(Condition::all().add(entity::user::Column::DeletedAt.is_null()))
        .order_by(entity::user::Column::CreatedAt, Order::Desc)
        .limit(Some(p.take()))
        .offset(Some(p.skip()))
        .all(conn)
        .await?;

    Ok(res)
}

pub async fn get_all() -> AppResult<Vec<User>> {
    let conn = seaorm_util::conn().await;

    let res = entity::user::Entity::find()
        .filter(Condition::all().add(entity::user::Column::DeletedAt.is_null()))
        .order_by(entity::user::Column::CreatedAt, Order::Desc)
        .all(conn)
        .await?;

    Ok(res)
}

pub async fn get(id: i64) -> AppResult<Option<User>> {
    let conn = seaorm_util::conn().await;

    let res = entity::user::Entity::find()
        .filter(
            Condition::all()
                .add(entity::user::Column::DeletedAt.is_null())
                .add(entity::user::Column::Id.eq(id)),
        )
        .order_by(entity::user::Column::CreatedAt, Order::Desc)
        .one(conn)
        .await?;

    Ok(res)
}

pub async fn get_by_email(email: &str) -> AppResult<Option<User>> {
    let conn = seaorm_util::conn().await;

    let res = entity::user::Entity::find()
        .filter(
            Condition::all()
                .add(entity::user::Column::DeletedAt.is_null())
                .add(entity::user::Column::Email.eq(email)),
        )
        .order_by(entity::user::Column::CreatedAt, Order::Desc)
        .one(conn)
        .await?;

    Ok(res)
}

pub async fn insert(
    email: &str,
    salt: &str,
    pwd: &str,
    name: Option<&str>,
    mobile: Option<&str>,
) -> AppResult<User> {
    let created_at = chrono::Local::now().with_timezone(&FixedOffset::east_opt(8 * 3600).unwrap());
    let user = entity::user::ActiveModel {
        id: ActiveValue::NotSet, // 不需要手动设置id
        email: Set(email.to_string()),
        salt: Set(salt.to_string()),
        pwd: Set(Some(pwd.to_string())),
        name: Set(name.map(|x| x.to_string())),
        mobile: Set(mobile.map(|x| x.to_string())),
        laston: Set(None),
        avatar: Set(None),
        created_at: Set(created_at),
        updated_at: Set(None),
        deleted_at: Set(None),
        r#type: Set(Usertype::Normal),
        status: Set(Userstatus::Available),
    };
    let conn = seaorm_util::conn().await;
    let insert_user = user.insert(conn).await?;

    let user_model = entity::user::Model::from(insert_user);
    Ok(user_model)
}

pub async fn delete(ids: &[i64]) -> AppResult<u64> {
    let deleted_at = chrono::Local::now().with_timezone(&FixedOffset::east_opt(8 * 3600).unwrap());
    let conn = seaorm_util::conn().await;
    let res = entity::user::Entity::update_many()
        .filter(entity::user::Column::Id.is_in(ids.to_owned()))
        .col_expr(entity::user::Column::DeletedAt, deleted_at.into())
        .exec(conn)
        .await?;

    Ok(res.rows_affected)
}

pub async fn update(id: i64, name: Option<&str>, mobile: Option<&str>) -> AppResult<User> {
    let updated_at = chrono::Local::now().with_timezone(&FixedOffset::east_opt(8 * 3600).unwrap());
    
    let user = entity::user::ActiveModel {
        id: Set(id), // 不更新id
        email: ActiveValue::NotSet,
        salt: ActiveValue::NotSet,
        pwd: ActiveValue::NotSet,
        name: name
            .map(|x| Set(Some(x.to_string())))
            .unwrap_or(ActiveValue::NotSet),
        mobile: mobile
            .map(|x| Set(Some(x.to_string())))
            .unwrap_or(ActiveValue::NotSet),
        laston: ActiveValue::NotSet,
        avatar: ActiveValue::NotSet,
        created_at: ActiveValue::NotSet,
        updated_at: Set(Some(updated_at)),
        deleted_at: ActiveValue::NotSet,
        r#type: ActiveValue::NotSet,
        status: ActiveValue::NotSet,
    };

    let conn = seaorm_util::conn().await;

    let res = user.update(conn).await?;

    Ok(res)
}

pub async fn update_pwd(id: i64, salt: &str, pwd: &str) -> AppResult<User> {
    let updated_at = chrono::Local::now().with_timezone(&FixedOffset::east_opt(8 * 3600).unwrap());
    let user = entity::user::ActiveModel {
        id: Set(id), // 不更新id
        email: ActiveValue::NotSet,
        salt: Set(salt.to_string()),
        pwd: Set(Some(pwd.to_string())),
        name: ActiveValue::NotSet,
        mobile: ActiveValue::NotSet,
        laston: ActiveValue::NotSet,
        avatar: ActiveValue::NotSet,
        created_at: ActiveValue::NotSet,
        updated_at: Set(Some(updated_at)),
        deleted_at: ActiveValue::NotSet,
        r#type: ActiveValue::NotSet,
        status: ActiveValue::NotSet,
    };

    let conn = seaorm_util::conn().await;

    let res = user.update(conn).await?;
    Ok(res)
}

pub async fn update_laston(id: i64, laston: &chrono::DateTime<chrono::Utc>) -> AppResult<User> {
    let laston = laston.with_timezone(&FixedOffset::east_opt(8 * 3600).unwrap());
    let user = entity::user::ActiveModel {
        id: Set(id), // 不更新id
        email: ActiveValue::NotSet,
        salt: ActiveValue::NotSet,
        pwd: ActiveValue::NotSet,
        name: ActiveValue::NotSet,
        mobile: ActiveValue::NotSet,
        laston: Set(Some(laston)),
        avatar: ActiveValue::NotSet,
        created_at: ActiveValue::NotSet,
        updated_at: ActiveValue::NotSet,
        deleted_at: ActiveValue::NotSet,
        r#type: ActiveValue::NotSet,
        status: ActiveValue::NotSet,
    };

    let conn = seaorm_util::conn().await;

    let res = user.update(conn).await?;
    Ok(res)
}
