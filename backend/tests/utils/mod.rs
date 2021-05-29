use backend::{self, run_migrations, DbPool};
use diesel::{pg::PgConnection, r2d2::ConnectionManager, result::Error, sql_query, RunQueryDsl};
use r2d2::{Pool, PooledConnection};
use std::env;

const PG_USER: &str = "tester";
const PG_PW: &str = "tester_password";
const PG_DB: &str = "test_db";

pub struct DbSetup {
    master_db_pool: DbPool,
    pub pool: DbPool,
}

impl DbSetup {
    pub fn new() -> Self {
        println!("🐨");
        let pg_user = env::var("POSTGRES_USER")
            .expect("POSTGRES_USER not set")
            .replace('"', "");
        let pg_password = env::var("POSTGRES_PASSWORD")
            .expect("POSTGRES_PASSWORD not set")
            .replace('"', "");
        let pg_db = env::var("POSTGRES_DB")
            .expect("POSTGRES_DB not set")
            .replace('"', "");
        let master_db_url = format!("postgres://{}:{}@database/{}", pg_user, pg_password, pg_db);

        let master_db_manager = ConnectionManager::<PgConnection>::new(master_db_url);
        let master_db_pool = Pool::builder()
            .max_size(1)
            .build(master_db_manager)
            .unwrap();

        let conn = master_db_pool.get().unwrap();
        DbSetup::clean_db(&conn).unwrap();
        sql_query(format!("CREATE DATABASE {};", PG_DB))
            .execute(&conn)
            .unwrap();

        sql_query(format!("CREATE USER {};", PG_USER))
            .execute(&conn)
            .unwrap();

        sql_query(format!(
            "ALTER USER {} WITH ENCRYPTED PASSWORD '{}';",
            PG_USER, PG_PW
        ))
        .execute(&conn)
        .unwrap();

        sql_query(format!(
            "GRANT ALL PRIVILEGES ON DATABASE {} TO {};",
            PG_DB, PG_USER
        ))
        .execute(&conn)
        .unwrap();

        let database_url = format!("postgres://{}:{}@database/{}", PG_USER, PG_PW, PG_DB);
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder().build(manager).unwrap();

        run_migrations(pool.clone());

        Self {
            master_db_pool,
            pool,
        }
    }

    fn clean_db(conn: &PooledConnection<ConnectionManager<PgConnection>>) -> Result<(), Error> {
        let pg_user = env::var("POSTGRES_USER")
            .expect("POSTGRES_USER not set")
            .replace('"', "");

        sql_query(format!("DROP DATABASE IF EXISTS {} WITH (FORCE);", PG_DB)).execute(conn)?;

        sql_query(format!("REASSIGN OWNED BY {} TO {};", PG_USER, pg_user)).execute(conn)?;
        sql_query(format!("DROP OWNED BY {};", PG_USER)).execute(conn)?;
        sql_query(format!("DROP USER IF EXISTS {};", PG_USER)).execute(conn)?;
        Ok(())
    }
}

impl Drop for DbSetup {
    fn drop(&mut self) {
        println!("🐱‍🏍");
        let conn = self.master_db_pool.get().unwrap();
        DbSetup::clean_db(&conn).unwrap();
    }
}
