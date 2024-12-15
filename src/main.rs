use std::sync::Arc;

use anyhow::Context;
use axum::{routing::get, Router};
use clap::Parser;
use config::{Config, DaoType};
use dao::{HashMapDao, MockedDao};
use http::{handlers, AppState};
use tokio::net::TcpListener;

mod config;
mod dao;
mod http;

#[tokio::main]
async fn main() {
    let args = Config::parse().runtime_args;

    let bind_address = format!("{}:{}", args.bind_host, args.bind_port);
    let listener = TcpListener::bind(&bind_address)
        .await
        .context(format!("Cannot bind to {bind_address}"))
        .unwrap();

    let state: AppState = match args.dao_type {
        DaoType::Mocked => Arc::new(MockedDao {}),
        DaoType::HashMap => Arc::new(HashMapDao::new()),
    };

    let router = Router::new()
        .route("/items", get(handlers::list).post(handlers::create))
        .route(
            "/items/:id",
            get(handlers::get)
                .put(handlers::update)
                .delete(handlers::delete),
        )
        .with_state(state);

    axum::serve(listener, router).await.unwrap();
}
