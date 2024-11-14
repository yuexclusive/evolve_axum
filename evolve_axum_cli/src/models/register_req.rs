/*
 * evolve_axum_new
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
pub struct RegisterReq {
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "mobile", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub mobile: Option<Option<String>>,
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    #[serde(rename = "pwd")]
    pub pwd: String,
}

impl RegisterReq {
    pub fn new(code: String, email: String, pwd: String) -> RegisterReq {
        RegisterReq {
            code,
            email,
            mobile: None,
            name: None,
            pwd,
        }
    }
}

