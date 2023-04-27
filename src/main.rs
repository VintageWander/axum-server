#![allow(dead_code, unused_variables)]

use std::net::SocketAddr;

use axum::{response::Response, Server};

use db::mongo::Mongo;
use dotenv::{dotenv, var};
use error::Error;

use service::Service;

use crate::routes::app_routes;

mod dao;
mod db;
mod error;
mod extractors;
mod handler;
mod helper;
mod model;
mod request;
mod routes;
mod service;
mod validation;
mod web;

type Result<T> = std::result::Result<T, Error>;
type WebResult = Result<Response>;

#[tokio::main]
async fn main() -> Result<()> {
    // Anything fails in main should crash, until all is resolved
    dotenv().ok();

    let db = Mongo::init().await;

    let port = var("PORT")
        .expect("Cannot read the PORT in the env")
        .parse()
        .expect("Cannot convert PORT variable into u16");

    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    let router = app_routes().with_state(Service::init(&db));

    Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();

    Ok(())
}
