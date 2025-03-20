/*
 * demo_server_main
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize, de::Error as _};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration, ContentType};


/// struct for typed errors of method [`download`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DownloadError {
    Status400(models::ErrorResp),
    Status500(models::ErrorResp),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`upload`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UploadError {
    Status400(models::ErrorResp),
    Status500(models::ErrorResp),
    UnknownValue(serde_json::Value),
}


pub async fn download(configuration: &configuration::Configuration, file_name: &str) -> Result<Vec<i32>, Error<DownloadError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_file_name = file_name;

    let uri_str = format!("{}/v1/file/download/{file_name}", configuration.base_path, file_name=crate::apis::urlencode(p_file_name));
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `Vec&lt;i32&gt;`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `Vec&lt;i32&gt;`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<DownloadError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn upload(configuration: &configuration::Configuration, file: std::path::PathBuf, name: &str) -> Result<models::FileUploadResp, Error<UploadError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_name = name;

    let uri_str = format!("{}/v1/file/upload", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::POST, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }
    let mut multipart_form = reqwest::multipart::Form::new();
    // TODO: support file upload for 'file' parameter
    multipart_form = multipart_form.text("name", p_name.to_string());
    req_builder = req_builder.multipart(multipart_form);

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::FileUploadResp`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::FileUploadResp`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<UploadError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

