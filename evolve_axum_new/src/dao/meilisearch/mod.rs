pub mod user;

use evolve_error::AppResult;
use crate::meilisearch_util::{self, Settings};
use serde::Serialize;
use std::fmt::Display;

pub const USER_LIST_INDEX: &str = "user_list";

pub async fn reload<D>(index: &str, documents: &[D], primary_key: Option<&str>) -> AppResult<()>
where
    D: Serialize,
{
    meilisearch_util::client()
        .index(index)
        .delete_all_documents()
        .await?;

    meilisearch_util::client()
        .index(index)
        .add_documents(documents, primary_key)
        .await?
        .wait_for_completion(meilisearch_util::client(), None, None)
        .await?;

    meilisearch_util::client()
        .index(index)
        .set_settings(&Settings::new().with_sortable_attributes(["created_at", "updated_at"]))
        .await?;

    Ok(())
}

pub async fn update<D>(index: &str, documents: &[D], primary_key: Option<&str>) -> AppResult<()>
where
    D: Serialize,
{
    meilisearch_util::client()
        .index(index)
        .add_or_update(documents, primary_key)
        .await?
        .wait_for_completion(meilisearch_util::client(), None, None)
        .await?;
    Ok(())
}

pub async fn delete<T>(index: &str, ids: &[T]) -> AppResult<()>
where
    T: Display + Serialize + std::fmt::Debug,
{
    meilisearch_util::client()
        .index(index)
        .delete_documents(ids)
        .await?
        .wait_for_completion(meilisearch_util::client(), None, None)
        .await?;

    Ok(())
}
