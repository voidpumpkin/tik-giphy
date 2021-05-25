use actix_web::{middleware::Logger, App, HttpServer};

mod resources;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(resources::users())
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}
