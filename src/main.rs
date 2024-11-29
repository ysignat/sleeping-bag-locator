#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
use std::env::var;

use anyhow::Context;
use axum::{routing::get, Router};
use constants::BIND_ADDRESS;
use http::{Crud, MockedCrud};
use tokio::net::TcpListener;

mod constants;
mod dao;
mod errors;
mod http;

#[tokio::main]
async fn main() {
    let port = var("PORT").unwrap_or(8080.to_string());
    let bind_address = format!("{BIND_ADDRESS}:{port}");
    let listener = TcpListener::bind(&bind_address)
        .await
        .context(format!("Cannot bind to {bind_address}"))
        .unwrap();

    // let router = Router::new()
    //     .route("/items", get(MockedCrud::list).post(MockedCrud::create))
    //     .route(
    //         "/items/:id",
    //         get(MockedCrud::get)
    //             .put(MockedCrud::update)
    //             .delete(MockedCrud::delete),
    //     );

    // axum::serve(listener, router).await.unwrap();
}
