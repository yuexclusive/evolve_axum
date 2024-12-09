/*
 * evolve_axum_main
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
pub struct UserInfoResp {
    #[serde(rename = "data")]
    pub data: Box<models::CurrentUser>,
}

impl UserInfoResp {
    pub fn new(data: models::CurrentUser) -> UserInfoResp {
        UserInfoResp {
            data: Box::new(data),
        }
    }
}

