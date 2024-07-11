// #![allow(unused)]

mod api;
mod api_doc;
mod chat;
mod dao;
mod datetime;
mod email;
mod env;
mod error;
mod meilisearch;
mod model;
mod postgres;
mod redis;
mod response;
mod service;

use crate::service::user as user_service;
use api::auth as api_jwt;
use api::user as api_user;
use api_doc::ApiDoc;
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
use tower_http::cors::{Any, CorsLayer};
use tower_http::{
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
    LatencyUnit,
};
use tracing::{instrument, Level};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable as ScalarServable};
use utoipa_swagger_ui::SwaggerUi;

use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};
use tokio::sync::broadcast;

use chat::{index, websocket_handler, AppState};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
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
    postgres::init();

    // init redis
    redis::init(
        &ENV.redis_host,
        ENV.redis_port,
        ENV.redis_username.clone(),
        ENV.redis_password.clone(),
    )
    .await;

    // init meilisearch
    meilisearch::init(&ENV.meilisearch_address, &ENV.meilisearch_api_key).await;

    // load user search data
    user_service::load_search().await?;

    // init mail
    email::init(
        &ENV.email_username,
        &ENV.email_password,
        &ENV.email_relay,
        ENV.email_port,
    )
    .await;

    // start servers
    tokio::join!(serve(web_service(), ENV.port),);

    Ok(())
}

async fn serve(app: Router, port: u16) {
    let listener = TcpListener::bind(SocketAddr::from((Ipv4Addr::UNSPECIFIED, port)))
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
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
    .await
    .unwrap();
}

fn web_service() -> Router {
    // init scalar
    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .merge(Scalar::with_url("/scalar", ApiDoc::openapi()));

    // build our application with a route
    let app = app
        .route("/ping", any(handler))
        .route("/auth/authorize", post(api_jwt::authorize))
        .route("/auth/user_info", get(api_jwt::user_info))
        .route("/api/user/send_email_code", post(api_user::send_email_code))
        .route("/api/user/change_pwd", post(api_user::change_pwd))
        .route("/api/user/register", post(api_user::register))
        .route(
            "/api/validate_exist_email/:email",
            get(api_user::validate_exist_email),
        )
        .route(
            "/api/validate_not_exist_email/:email",
            get(api_user::validate_not_exist_email),
        )
        .route("/api/user/search", get(api_user::search))
        .route("/api/user/:id", get(api_user::get))
        .route("/api/user/update", put(api_user::update))
        .route("/api/user/delete", delete(api_user::delete))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );

    // websocket
    let user_set = Mutex::new(HashSet::new());
    let (tx, _rx) = broadcast::channel(100);

    let app_state = Arc::new(AppState { user_set, tx });

    let app = app
        .route("/chat", get(index))
        .route("/websocket", get(websocket_handler))
        .with_state(app_state);

    app
}

#[instrument]
async fn handler() -> &'static str {
    "Pong"
}
