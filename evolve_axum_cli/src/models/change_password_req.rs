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
pub struct ChangePasswordReq {
    /// validate code
    #[serde(rename = "code")]
    pub code: String,
    /// email
    #[serde(rename = "email")]
    pub email: String,
    /// password
    #[serde(rename = "pwd")]
    pub pwd: String,
}

impl ChangePasswordReq {
    pub fn new(code: String, email: String, pwd: String) -> ChangePasswordReq {
        ChangePasswordReq {
            code,
            email,
            pwd,
        }
    }
}
