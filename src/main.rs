#![allow(dead_code, unused_variables)]

use std::net::SocketAddr;

use axum::{
    http::{
        header::{
            ACCEPT, ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS,
            ACCESS_CONTROL_ALLOW_ORIGIN, CONTENT_TYPE, ORIGIN,
        },
        HeaderValue, Method,
    },
    response::Response,
    Server,
};

use db::mongo::Mongo;
use dotenv::{dotenv, var};
use error::Error;

use service::Service;
use tower_http::cors::CorsLayer;

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

    let origin = var("ORIGIN").expect("Cannot read the ORIGIN in the env");

    let router = app_routes().with_state(Service::init(&db)).layer(
        CorsLayer::new()
            .allow_credentials(true)
            .allow_origin(
                origin
                    .parse::<HeaderValue>()
                    .expect("Failed to parse origin as HeaderValue"),
            )
            .allow_headers([
                ORIGIN,
                CONTENT_TYPE,
                ACCEPT,
                ACCESS_CONTROL_ALLOW_ORIGIN,
                ACCESS_CONTROL_ALLOW_METHODS,
                ACCESS_CONTROL_ALLOW_HEADERS,
            ])
            .allow_methods([
                Method::GET,
                Method::POST,
                Method::PUT,
                Method::DELETE,
                Method::OPTIONS,
            ]),
    );

    let port = var("PORT")
        .expect("Cannot read the PORT in the env")
        .parse()
        .expect("Cannot convert PORT variable into u16");

    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();

    Ok(())
}
