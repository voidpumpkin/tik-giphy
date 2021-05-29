use crate::{models, utils::SuccessfulResponseBody, DbPool};
use actix_web::{get, post, web, HttpResponse, Responder, Scope};
use diesel::RunQueryDsl;

pub fn users() -> Scope {
    web::scope("/users")
        .service(get_users)
        .service(get_me)
        .service(post_user)
}

#[get("/")]
async fn get_users(pool: web::Data<DbPool>) -> impl Responder {
    let db = pool.get().expect("couldn't get db connection from pool");

    let users_result = web::block(
        move || -> Result<Vec<models::User>, diesel::result::Error> {
            use crate::schema::users::dsl::*;
            let user = users.load::<models::User>(&db)?;
            Ok(user)
        },
    )
    .await;

    match users_result {
        Ok(users) => HttpResponse::Ok().json(SuccessfulResponseBody { data: users }),
        Err(err) => {
            eprint!("{}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/me")]
async fn get_me() -> impl Responder {
    HttpResponse::Ok().body("I am a you")
}

#[post("/")]
async fn post_user(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(format!("You posted user: {}", req_body))
}
