use std::sync::Arc;

use axum::{routing::get, Router};
use clap::Parser;
use config::{Config, DaoType, LogFormat};
use dao::{ItemsHashMapDao, ItemsMockedDao};
use http::{create_item, delete_item, get_item, health, list_items, update_item, AppState};
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
    let listener = match TcpListener::bind(&bind_address).await {
        Err(err) => {
            error!(
                target : TRACING_STARTUP_TARGET,
                "Cannot bind to {bind_address}: {err}"
            );
            panic!()
        }
        Ok(listener) => {
            info!(
                target : TRACING_STARTUP_TARGET,
                "Created listener at {bind_address}"
            );
            listener
        }
    };

    let state: AppState = match args.runtime.dao_type {
        DaoType::Mocked => {
            info!(target : TRACING_STARTUP_TARGET, "Using MockedDao");
            AppState {
                items: Arc::new(ItemsMockedDao {}),
            }
        }
        DaoType::HashMap => {
            info!(target : TRACING_STARTUP_TARGET, "Using HashMapDao");
            AppState {
                items: Arc::new(ItemsHashMapDao::new()),
            }
        }
    };

    let router = Router::new()
        .route("/items", get(list_items).post(create_item))
        .route(
            "/items/:id",
            get(get_item).put(update_item).delete(delete_item),
        )
        .layer(TraceLayer::new_for_http())
        .route("/health", get(health))
        .with_state(state);
    info!(target : TRACING_STARTUP_TARGET, "Created router");

    info!(target : TRACING_STARTUP_TARGET, "Starting server");
    if let Err(err) = axum::serve(listener, router).await {
        error!(
            target : TRACING_STARTUP_TARGET,
            "Failed to start server: {err}"
        );
        panic!()
    }
}
