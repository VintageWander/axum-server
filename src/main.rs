#![allow(dead_code, unused_variables)]

use error::Error;

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

#[tokio::main]
async fn main() {
    println!("Hello, world!");
}
