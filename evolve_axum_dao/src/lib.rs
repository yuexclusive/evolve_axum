#![allow(unused)]

use serde::Deserialize;
use utoipa::IntoParams;
pub mod meilisearch;
pub mod model;
pub mod pg;
pub mod redis;

#[derive(Deserialize)]
pub struct Paging {
    pub page_index: i64,
    pub page_size: i64,
}

impl Paging {
    pub fn skip(&self) -> i64 {
        self.page_index.checked_sub(1).unwrap_or(0) * self.page_size
    }

    pub fn take(&self) -> i64 {
        self.page_size.max(0)
    }
}