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
pub struct AuthReq {
    /// authorize
    #[serde(rename = "authorize_type")]
    pub authorize_type: models::AuthorizeType,
    /// id
    #[serde(rename = "id")]
    pub id: String,
    /// secret
    #[serde(rename = "secret")]
    pub secret: String,
}

impl AuthReq {
    pub fn new(authorize_type: models::AuthorizeType, id: String, secret: String) -> AuthReq {
        AuthReq {
            authorize_type,
            id,
            secret,
        }
    }
}
