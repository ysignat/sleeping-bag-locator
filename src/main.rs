use std::sync::Arc;

use async_redis_session::RedisSessionStore;
use async_session::MemoryStore;
use axum::{routing::get, Router};
use clap::Parser;
use config::{Config, DaoType, LogFormat, SessionStoreType};
use dao::{ItemsHashMapDao, ItemsMockedDao};
use http::{
    auth_callback,
    create_item,
    delete_item,
    get_item,
    health,
    list_items,
    login,
    logout,
    update_item,
    AppState,
};
use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, TokenUrl};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::{error, info};

mod config;
mod dao;
mod http;

const TRACING_STARTUP_TARGET: &str = "startup";

#[tokio::main]
async fn main() {
    let args = Config::parse();

    let subscriber_builder = tracing_subscriber::fmt().with_max_level(args.logging.log_level);

    match args.logging.log_format {
        LogFormat::Default => subscriber_builder.init(),
        LogFormat::Json => subscriber_builder.json().init(),
        LogFormat::Pretty => subscriber_builder.pretty().init(),
    }

    info!(
        target : TRACING_STARTUP_TARGET,
        "Tracing subscriber started with log level {} and {:?} log format", args.logging.log_level, args.logging.log_format,
    );

    let bind_address = format!("{}:{}", args.runtime.bind_host, args.runtime.bind_port);
    let listener = TcpListener::bind(&bind_address)
        .await
        .inspect_err(|err| {
            error!(
                target : TRACING_STARTUP_TARGET,
                "Cannot bind to {bind_address}: {err}"
            );
        })
        .unwrap();
    info!(
        target : TRACING_STARTUP_TARGET,
        "Created listener at {bind_address}"
    );

    let oauth = BasicClient::new(ClientId::new(args.authentication.oauth_client_id))
        .set_client_secret(ClientSecret::new(args.authentication.oauth_client_secret))
        .set_auth_uri(AuthUrl::new("https://github.com/login/oauth/authorize".to_owned()).unwrap())
        .set_token_uri(
            TokenUrl::new("https://github.com/login/oauth/access_token".to_owned()).unwrap(),
        );

    let state = AppState {
        items: match args.runtime.dao_type {
            DaoType::Mocked => {
                info!(target : TRACING_STARTUP_TARGET, "Using ItemsMockedDao");
                Arc::new(ItemsMockedDao {})
            }
            DaoType::HashMap => {
                info!(target : TRACING_STARTUP_TARGET, "Using ItemsHashMapDao");
                Arc::new(ItemsHashMapDao::new())
            }
        },
        session_store: match args.session_store.session_store_type {
            SessionStoreType::Memory => {
                info!(target : TRACING_STARTUP_TARGET, "Using MemoryStore");
                Arc::new(MemoryStore::new())
            }
            SessionStoreType::Redis => {
                info!(target : TRACING_STARTUP_TARGET, "Using RedisSessionStore");
                if args.session_store.session_store_dsn.is_empty() {
                    error!(target: TRACING_STARTUP_TARGET, "Cannot instantiate RedisSessionStore with empty DSN");
                    panic!()
                }
                let session_store = RedisSessionStore::new(
                    args.session_store.session_store_dsn.clone(),
                ).inspect_err(
                    |err|
                    error!(target: TRACING_STARTUP_TARGET, "Error while creating RedisSessionStore: {err:#?}")
                ).unwrap();
                info!(target : TRACING_STARTUP_TARGET, "Created RedisSessionStore with {:#?}", args.session_store.session_store_dsn);
                Arc::new(session_store)
            }
        },
        oauth,
    };

    let router = Router::new()
        .route("/items", get(list_items).post(create_item))
        .route(
            "/items/:id",
            get(get_item).put(update_item).delete(delete_item),
        )
        .layer(TraceLayer::new_for_http())
        .route("/health", get(health))
        .route("/login", get(login))
        .route("/auth/callback", get(auth_callback))
        .route("/logout", get(logout))
        .with_state(state);
    info!(target : TRACING_STARTUP_TARGET, "Created router");

    info!(target : TRACING_STARTUP_TARGET, "Starting server");
    axum::serve(listener, router)
        .await
        .inspect_err(|err| {
            error!(
                target : TRACING_STARTUP_TARGET,
                "Failed to start server: {err}"
            );
        })
        .unwrap();
}
