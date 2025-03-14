use serde::Deserialize;

#[derive(Deserialize)]
pub struct Paging {
    pub page_index: u64,
    pub page_size: u64,
}

impl Paging {
    pub fn skip(&self) -> u64 {
        self.page_index.checked_sub(1).unwrap_or(0) * self.page_size
    }

    pub fn take(&self) -> u64 {
        self.page_size.max(0)
    }
}