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
pub struct UserUpdateResp {
    #[serde(rename = "data")]
    pub data: Box<models::User>,
}

impl UserUpdateResp {
    pub fn new(data: models::User) -> UserUpdateResp {
        UserUpdateResp {
            data: Box::new(data),
        }
    }
}
