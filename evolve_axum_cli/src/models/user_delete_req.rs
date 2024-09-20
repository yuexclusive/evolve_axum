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
pub struct UserDeleteReq {
    #[serde(rename = "ids")]
    pub ids: Vec<i64>,
}

impl UserDeleteReq {
    pub fn new(ids: Vec<i64>) -> UserDeleteReq {
        UserDeleteReq {
            ids,
        }
    }
}

