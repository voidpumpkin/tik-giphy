use actix_web::{middleware::Logger, App, HttpServer};
use std::env;

mod resources;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("INFO"));

    let host: String = env::var("BACKEND_HOST").unwrap_or("127.0.0.1:8081".into());

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(resources::users())
    })
    .bind(host)?
    .run()
    .await
}
