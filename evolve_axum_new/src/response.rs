// use serde::Deserialize;
// use utoipa::IntoParams;

// #[derive(Deserialize, IntoParams)]
// pub struct Pagination {
//     #[param(example = 1)]
//     pub index: i64,
//     #[param(example = 20)]
//     pub size: i64,
// }

// impl Pagination {

//     pub fn skip(&self) -> i64 {
//         self.index.checked_sub(1).unwrap_or(0) * self.size
//     }

//     pub fn take(&self) -> i64 {
//         self.size.max(0)
//     }
// }
