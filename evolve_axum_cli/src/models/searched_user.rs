/*
 * evolve_axum
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchedUser {
    #[serde(rename = "formatter")]
    pub formatter: Box<models::UserFormatter>,
    #[serde(rename = "user")]
    pub user: Box<models::User>,
}

impl SearchedUser {
    pub fn new(formatter: models::UserFormatter, user: models::User) -> SearchedUser {
        SearchedUser {
            formatter: Box::new(formatter),
            user: Box::new(user),
        }
    }
}

