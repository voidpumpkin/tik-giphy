#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate lazy_static;

embed_migrations!();

use actix_web::{
    middleware::{Logger, NormalizePath},
    App, HttpServer,
};
use diesel::{pg::PgConnection, r2d2::ConnectionManager};
use std::env;

mod config;
mod middlewares;
pub mod models;
mod resources;
pub mod schema;
pub mod utils;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("INFO"));

    let host: String = env::var("BACKEND_HOST").unwrap_or("127.0.0.1:8081".into());
    let database_url = env::var("DATABASE_URL").unwrap();

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Could not build connection pool");

    let connection = pool.get().unwrap();
    embedded_migrations::run(&connection).unwrap();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(Logger::default())
            .wrap(NormalizePath::default())
            .service(services_config!())
    })
    .bind(host)?
    .run()
    .await
}
