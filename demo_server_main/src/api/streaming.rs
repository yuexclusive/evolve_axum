use async_stream::stream;
use axum::{http::StatusCode, response::IntoResponse};
use std::time::Duration;
use tokio_stream::StreamExt;

pub async fn streaming() -> impl IntoResponse {
    let stream: async_stream::__private::AsyncStream<Result<String, std::io::Error>, _> = stream! {
        for i in 0..10 {
            yield Ok(format!("data: {}\n\n", format!("Chunk {}", i)));
        }
    };

    let headers = [
        (axum::http::header::CACHE_CONTROL, "no-cache"),
        (axum::http::header::CONTENT_TYPE, "text/event-stream"),
        (axum::http::header::CONNECTION, "keep-alive"),
    ];

    let throttled_stream = stream.throttle(Duration::from_secs(1));

    (
        StatusCode::OK,
        headers,
        axum::body::Body::from_stream(throttled_stream),
    )
}
