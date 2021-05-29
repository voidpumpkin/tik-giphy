use actix_web::test;
use backend::{self, create_app, models};
use diesel::{sql_query, RunQueryDsl};

mod utils;

#[actix_rt::test]
async fn gets_no_users() {
    let db_setup = utils::DbSetup::new();

    let conn = db_setup.pool.get().unwrap();
    sql_query("DELETE FROM users;").execute(&conn).unwrap();

    let mut app = test::init_service(create_app!(db_setup.pool.clone())).await;

    let req = test::TestRequest::get().uri("/users/").to_request();
    let resp = test::call_service(&mut app, req).await;
    let users: Vec<models::User> = test::read_body_json(resp).await;

    assert_eq!(users, vec![] as Vec<models::User>);
}

#[actix_rt::test]
async fn gets_two_users() {
    let db_setup = utils::DbSetup::new();

    let conn = db_setup.pool.get().unwrap();
    sql_query("DELETE FROM users;").execute(&conn).unwrap();
    let db_users = sql_query(
        "\
INSERT INTO
     users (username, email, password)
VALUES
    ('tester', 'tester@tester.com', '123456'),
    ('bob', 'bob@tester.com', 'bobbyiscool')
RETURNING *;",
    )
    .load(&conn)
    .unwrap();

    let mut app = test::init_service(create_app!(db_setup.pool.clone())).await;

    let req = test::TestRequest::get().uri("/users/").to_request();
    let resp = test::call_service(&mut app, req).await;
    let users: Vec<models::User> = test::read_body_json(resp).await;

    assert_eq!(users, db_users);
}
