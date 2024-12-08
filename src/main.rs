#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
use std::{env::var, sync::Arc};

use anyhow::Context;
use axum::{routing::get, Router};
use dao::{HashMapDao, MockedDao};
use http::{handlers, AppState};
use tokio::net::TcpListener;

mod dao;
mod http;

pub const BIND_ADDRESS: &str = "0.0.0.0";

#[tokio::main]
async fn main() {
    let port = var("PORT").unwrap_or(8080.to_string());
    let bind_address = format!("{BIND_ADDRESS}:{port}");
    let listener = TcpListener::bind(&bind_address)
        .await
        .context(format!("Cannot bind to {bind_address}"))
        .unwrap();

    let dao_type = var("DAO_TYPE").unwrap_or("HASH_MAP".to_owned());

    let state: AppState = if dao_type.eq("MOCKED") {
        Arc::new(MockedDao {})
    } else if dao_type.eq("HASH_MAP") {
        Arc::new(HashMapDao::new())
    } else {
        panic!("No DAO found for this type")
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
