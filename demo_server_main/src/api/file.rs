use std::path::PathBuf;

use axum::{
    body::Body,
    extract::{Multipart, Path},
    http::{header, StatusCode},
    response::Response,
    Json,
};
use demo_server_error::{AppError, ErrorResp};
use serde::{Deserialize, Serialize};
use tokio_util::io::ReaderStream;
use utoipa::{OpenApi, ToSchema};

// 定义最大文件大小为 10MB
const MAX_FILE_SIZE: usize = 10 * 1024 * 1024;

/// Just a schema for axum native multipart
#[derive(Deserialize, ToSchema)]
#[allow(unused)]
struct FileUploadReq {
    name: String,
    #[schema(format = Binary,  content_media_type = "application/octet-stream")]
    file: String,
}

#[derive(ToSchema, Serialize)]
pub struct FileUploadResp {
    name: Option<String>,
    files: Vec<FileInfo>,
}

#[derive(ToSchema, Serialize)]
pub struct FileInfo {
    file_name: String,
    content_type: String,
    size: usize,
}

#[utoipa::path(
    post,
    path = "/upload",
    request_body(content = inline(FileUploadReq), content_type = "multipart/form-data"),
    responses (
        (status = 200, description = "successfully", body = FileUploadResp),
        (status = 400, description = "bad request", body = ErrorResp),
        (status = 500, description = "internal server error", body = ErrorResp)
    )
)]
pub async fn upload(mut multipart: Multipart) -> Result<Json<FileUploadResp>, AppError> {
    let mut name: Option<String> = None;

    let mut files: Vec<FileInfo> = Vec::new();

    while let Some(field) = multipart.next_field().await? {
        let field_name = field.name();

        match &field_name {
            Some("name") => {
                name = Some(field.text().await?);
            }
            Some("file") => {
                let file_name = field.file_name().map(ToString::to_string);
                let content_type = field.content_type().map(ToString::to_string);
                let bytes = field.bytes().await?;
                let size = bytes.len();
                if size > MAX_FILE_SIZE {
                    return Err(AppError::Validate {
                        msg: format!(
                            "file too large, max size is {}MB",
                            MAX_FILE_SIZE / 1024 / 1024
                        ),
                    }
                    .into());
                }

                files.push(FileInfo {
                    file_name: file_name.clone().unwrap_or_default(),
                    content_type: content_type.clone().unwrap_or_default(),
                    size,
                });
            }
            _ => (),
        };
    }
    Ok(Json(FileUploadResp { name, files }))
}

// 文件下载端点
#[utoipa::path(
    get,
    path = "/download/{file_name}",
    params(
        ("file_name", description = "file_name")
    ),
    responses(
        (
            status = 200,
            description = "download file",
            content_type = "application/octet-stream",
            body = Vec<u8>,
            headers(
                ("Content-Disposition" = String, description = "file name of the downloaded file")
            )
        ),
        (status = 400, description = "bad request", body = ErrorResp),
        (status = 500, description = "internal server error", body = ErrorResp)
    )
)]
pub async fn download(
    Path((_version, file_name)): Path<(String, String)>,
) -> Result<impl axum::response::IntoResponse, AppError> {
    // 示例文件路径（实际应从存储系统获取）
    let file_path = PathBuf::from("/Users/yu").join(&file_name);

    // 验证文件存在
    if !file_path.exists() {
        return Err(AppError::NotFound {
            msg: format!("file {} not found", file_name),
        });
    }

    // 打开文件流
    let file = tokio::fs::File::open(&file_path)
        .await
        .map_err(|err| AppError::Internal {
            msg: format!("cannot open file: {}", err),
        })?;

    // 创建流式响应
    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/octet-stream")
        .header(
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{}\"", file_name),
        )
        .body(body)
        .unwrap())
}

#[derive(OpenApi)]
#[openapi(
    paths(upload, download),
    components(schemas(FileUploadResp, FileUploadReq, ErrorResp, FileInfo))
)]
pub struct FileApi;
