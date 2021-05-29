use actix_web::HttpServer;
use diesel::{pg::PgConnection, r2d2::ConnectionManager};
use std::env;

use backend::create_app;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("INFO"));

    let host: String = env::var("BACKEND_HOST").unwrap_or("127.0.0.1:8081".into());
    let database_url = env::var("DATABASE_URL").expect("Env variable BACKEND_HOST not specified");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Could not build connection pool");

    #[cfg(not(debug_assertions))]
    {
        backend::run_migrations(pool.clone());
    }

    HttpServer::new(move || create_app!(pool.clone()))
        .bind(host)?
        .run()
        .await
}
