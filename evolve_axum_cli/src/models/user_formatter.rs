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
pub struct UserFormatter {
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "laston")]
    pub laston: String,
    #[serde(rename = "mobile")]
    pub mobile: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

impl UserFormatter {
    pub fn new(created_at: String, email: String, laston: String, mobile: String, name: String, status: String, r#type: String, updated_at: String) -> UserFormatter {
        UserFormatter {
            created_at,
            email,
            laston,
            mobile,
            name,
            status,
            r#type,
            updated_at,
        }
    }
}

