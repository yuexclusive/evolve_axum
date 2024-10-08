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
pub struct SendEmailCodeReq {
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "from")]
    pub from: models::SendEmailCodeFrom,
}

impl SendEmailCodeReq {
    pub fn new(email: String, from: models::SendEmailCodeFrom) -> SendEmailCodeReq {
        SendEmailCodeReq {
            email,
            from,
        }
    }
}

