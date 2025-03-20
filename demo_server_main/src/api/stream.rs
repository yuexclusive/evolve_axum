use axum::response::sse::{Event, Sse};
use axum_extra::{headers, TypedHeader};
use futures::stream::{self, Stream};
use std::time::Duration;
use tokio_stream::StreamExt;
use utoipa::OpenApi;

#[utoipa::path(
    get,
    path = "/streaming",
    responses(
        (status = 200, description = "Server-Sent Events stream", content_type = "text/event-stream"),
    ),
)]
pub async fn streaming(
    TypedHeader(user_agent): TypedHeader<headers::UserAgent>,
) -> Sse<impl Stream<Item = Result<Event, std::convert::Infallible>>> {
    println!("`{}` connected", user_agent.as_str());

    let items = vec![
        serde_json::json!({"id": 1, "name": "Alice"}),
        serde_json::json!({"id": 2, "name": "Bob"}),
        serde_json::json!({"id": 3, "name": "Charlie"}),
    ];

    let stream = stream::iter(items)
        .map(|item| {
            let data = serde_json::to_string(&item).unwrap();
            Ok(Event::default().data(data))
        })
        .throttle(Duration::from_secs(1)); // send event every second

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(1))
            .text("keep-alive-text"),
    )
}

#[derive(OpenApi)]
#[openapi(paths(streaming), components(schemas()))]
pub struct StreamApi;
