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
pub struct SendEmailCodeResp {
    #[serde(rename = "data")]
    pub data: i64,
}

impl SendEmailCodeResp {
    pub fn new(data: i64) -> SendEmailCodeResp {
        SendEmailCodeResp {
            data,
        }
    }
}

