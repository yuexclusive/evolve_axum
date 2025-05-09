/*
 * demo_server_main
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
pub struct AuthResp {
    #[serde(rename = "access_token")]
    pub access_token: String,
    #[serde(rename = "expired_on")]
    pub expired_on: i32,
}

impl AuthResp {
    pub fn new(access_token: String, expired_on: i32) -> AuthResp {
        AuthResp {
            access_token,
            expired_on,
        }
    }
}

