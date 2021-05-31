use diesel::{pg::PgConnection, r2d2::ConnectionManager};

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
