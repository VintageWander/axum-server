#![allow(dead_code, unused_variables)]

use std::net::SocketAddr;

use axum::{response::Response, Router, Server};
use dotenv::{dotenv, var};
use error::Error;
use services::user::UserService;

use crate::{db::mongo::Mongo, routes::user::user_routes};

mod dao;
mod db;
mod error;
mod handler;
mod helper;
mod model;
mod repo;
mod routes;
mod services;
mod validation;
pub mod web;

type Result<T> = std::result::Result<T, Error>;
type WebResult = Result<Response>;

#[derive(Debug, Clone)]
pub struct SharedState {
    user_service: UserService,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Anything fails in main should crash, until all is resolved
    dotenv().ok();

    let db = Mongo::init().await?;
    let user_service = UserService::init(&db);

    let port = var("PORT")
        .expect("Cannot read the PORT in the env")
        .parse()
        .expect("Cannot convert PORT variable into u16");

    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    let router = Router::new()
        .merge(user_routes())
        .with_state(SharedState { user_service });

    Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();

    println!("Hello, world!");

    Ok(())
}
