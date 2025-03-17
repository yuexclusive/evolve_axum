mod api;
mod api_doc;
// mod dao;
mod env;
mod service;

use crate::service::user as user_service;
use api::auth as api_jwt;
use api::user as api_user;
use api_doc::ApiDoc;
use axum::Json;
use axum::{
    routing::any,
    routing::{delete, get, post, put},
    Router,
};
use env::ENV;
use std::{
    error::Error,
    net::{Ipv4Addr, SocketAddr},
};
use tokio::net::TcpListener;
use tokio::runtime::Runtime;
use tokio::signal;
use tower_http::cors::{Any, CorsLayer};
use tower_http::{
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
    LatencyUnit,
};
use tracing::{instrument, Level};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
// use utoipa_rapidoc::RapiDoc;
// use utoipa_redoc::{Redoc, Servable};
use utoipa_scalar::{Scalar, Servable as ScalarServable};
use utoipa_swagger_ui::SwaggerUi;

use std::sync::Arc;

use tower_http::services::ServeDir;

use evolve_ws::{
    chat_redis::{self, WSState},
    store_redis::RedisStore,
};

#[no_mangle]
pub fn start_server_sync() -> Result<(), Box<dyn Error>> {
    let rt = Runtime::new()?;
    rt.block_on(async { start_server().await.unwrap() });
    Ok(())
}

pub async fn start_server() -> Result<(), Box<dyn Error>> {
    // init dotenv
    dotenv::dotenv().map_err(|e| format!("dotenv init failed: {:?}", e))?;

    // init tracing
    let rust_log = ENV.rust_log.clone();
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(move |e| {
                println!("tracing from default env error: {}", e);
                rust_log.into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // init postgres connection
    evolve_seaorm::init();

    // init redis
    evolve_redis::init(
        &ENV.redis_host,
        ENV.redis_port,
        ENV.redis_username.clone(),
        ENV.redis_password.clone(),
    )
    .await;

    // init meilisearch
    evolve_meilisearch::init(&ENV.meilisearch_address, &ENV.meilisearch_api_key).await;

    // load user search data
    user_service::load_search().await?;

    // // init mail
    // evolve_mailer::init(
    //     &ENV.email_username,
    //     &ENV.email_password,
    //     &ENV.email_relay,
    //     ENV.email_port,
    // )
    // .await;

    // start servers
    tokio::join!(
        serve(web_server(), 3000, true),
        // serve(dir_server(), 3001, false),
    );

    Ok(())
}

async fn serve(app: Router, port: u16, clear_sessions: bool) {
    let listener = TcpListener::bind(SocketAddr::from((Ipv4Addr::UNSPECIFIED, port)))
        .await
        .unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(
        listener,
        app.layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().include_headers(true))
                .on_request(DefaultOnRequest::new().level(Level::DEBUG))
                .on_response(
                    DefaultOnResponse::new()
                        .level(Level::DEBUG)
                        .latency_unit(LatencyUnit::Millis),
                ),
        ),
    )
    .with_graceful_shutdown(shutdown_signal(clear_sessions))
    .await
    .unwrap();
}

async fn shutdown_signal(clear_sessions: bool) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    let clear_ws_sessions = || {
        if clear_sessions {
            evolve_ws::excute_lua_script::clear_sessions().unwrap();
            tracing::info!("clear ws sessions done");
        }
    };

    tokio::select! {
        _ = ctrl_c => {
            tracing::info!("Ctrl+C signal received");
            clear_ws_sessions();
        },
        _ = terminate => {
            tracing::info!("terminate signal received");
            clear_ws_sessions();
        },
    }
}

#[allow(unused)]
fn dir_server() -> Router {
    // serve the file in the "assets" directory under `/assets`
    Router::new().nest_service("/static", ServeDir::new("static"))
}

fn web_server() -> Router {
    let api_routes = Router::new()
        .route("/auth/authorize", post(api_jwt::authorize))
        .route("/auth/user_info", get(api_jwt::user_info))
        .route("/user/send_email_code", post(api_user::send_email_code))
        .route("/user/change_pwd", post(api_user::change_pwd))
        .route("/user/register", post(api_user::register))
        .route(
            "/user/validate_exist_email/{email}",
            get(api_user::validate_exist_email),
        )
        .route(
            "/user/validate_not_exist_email/{email}",
            get(api_user::validate_not_exist_email),
        )
        .route("/user/search", get(api_user::search))
        .route("/user/{id}", get(api_user::get))
        .route("/user/update", put(api_user::update))
        .route("/user/delete", delete(api_user::delete))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );

    let ws_state = Arc::new(WSState::new(RedisStore::default()));

    let ws_routes = Router::new()
        .route("/chat", get(chat_redis::index))
        .route("/websocket", get(chat_redis::websocket_handler))
        .with_state(ws_state);

    let app = Router::new()
        .route("/ping", any(ping))
        .route("/ping_json", any(ping_json))
        .nest("/{version}", api_routes)
        .merge(ws_routes)
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        // .merge(Redoc::with_url("/redoc", ApiDoc::openapi()))
        // .merge(RapiDoc::new("/api-docs/openapi.json").path("/rapidoc"));
        .merge(Scalar::with_url("/scalar", ApiDoc::openapi()));

    app
}

#[instrument]
async fn ping() -> &'static str {
    "Pong"
}

#[instrument]
async fn ping_json(Json(data): Json<serde_json::Value>) -> Result<Json<serde_json::Value>, ()> {
    Ok(Json(serde_json::json!({ "data": data })))
}

/// examples of axum http api test
#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use http_body_util::BodyExt; // for `collect`
    use tower::ServiceExt; // for `call`, `oneshot`, and `ready`

    #[tokio::test]
    async fn ping() {
        let router = web_server();
        // `Router` implements `tower::Service<Request<Body>>` so we can
        // call it like any tower service, no need to run an HTTP server.
        let response = router
            .oneshot(Request::builder().uri("/ping").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let s = String::from_utf8_lossy(&body).to_string();
        assert_eq!(&s, "Pong");
    }

    #[tokio::test]
    async fn ping_json() {
        let router = web_server();
        // `Router` implements `tower::Service<Request<Body>>` so we can
        // call it like any tower service, no need to run an HTTP server.
        let response = router
            .oneshot(
                Request::builder()
                    .uri("/ping_json")
                    .header("Content-Type", "application/json")
                    .body(Body::from(r#"{"data": "data"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let s = String::from_utf8_lossy(&body).to_string();
        assert_eq!(&s, r#"{"data":{"data":"data"}}"#);
    }
}
