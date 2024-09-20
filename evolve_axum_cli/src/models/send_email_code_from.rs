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

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SendEmailCodeFrom {
    #[serde(rename = "Register")]
    Register,
    #[serde(rename = "ChangePwd")]
    ChangePwd,

}

impl std::fmt::Display for SendEmailCodeFrom {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Register => write!(f, "Register"),
            Self::ChangePwd => write!(f, "ChangePwd"),
        }
    }
}

impl Default for SendEmailCodeFrom {
    fn default() -> SendEmailCodeFrom {
        Self::Register
    }
}

