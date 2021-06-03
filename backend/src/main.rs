#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate lazy_static;

embed_migrations!();

mod config;
mod constants;
mod controllers;
mod middlewares;
mod models;
mod schema;
mod services;
mod types;
mod views;

use actix_web::HttpServer;
use constants::{DEFAULT_BACKEND_HOST, ENV_KEY_BACKEND_HOST, ENV_KEY_DATABASE_URL, RUST_LOG_INFO};
use diesel::{pg::PgConnection, r2d2::ConnectionManager};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or(RUST_LOG_INFO));

    let host: String = env::var(ENV_KEY_BACKEND_HOST).unwrap_or(DEFAULT_BACKEND_HOST.into());
    let database_url = env::var(ENV_KEY_DATABASE_URL).unwrap();

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder().build(manager).unwrap();

    let connection = pool.get().unwrap();
    embedded_migrations::run(&connection).unwrap();

    HttpServer::new(move || app_config!(pool.clone()))
        .bind(host)?
        .run()
        .await
}
