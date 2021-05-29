#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

embed_migrations!();

use diesel::{pg::PgConnection, r2d2::ConnectionManager};

pub mod models;
pub mod resources;
pub mod schema;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn run_migrations(pool: DbPool) {
    let connection = pool.get().unwrap();
    embedded_migrations::run(&connection).unwrap();
}

#[macro_export]
macro_rules! create_app {
    ($p:expr) => {
        actix_web::App::new()
            .data($p)
            .wrap(actix_web::middleware::Logger::default())
            .service(backend::resources::users())
    };
}
