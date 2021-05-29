use crate::{
    models,
    utils::{ErrorResponseBody, JsonApiError, SuccessfulResponseBody},
    DbPool,
};
use actix_threadpool::BlockingError;
use actix_web::{get, post, web, HttpResponse, Responder, Scope};
use diesel::{
    insert_into,
    result::{DatabaseErrorKind, Error::DatabaseError},
    RunQueryDsl,
};
use serde::Deserialize;

pub fn users() -> Scope {
    web::scope("/users").service(get_users).service(post_user)
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

#[derive(Deserialize)]
struct PostUserReqBody {
    username: String,
    email: String,
    password: String,
}

#[post("/")]
async fn post_user(
    pool: web::Data<DbPool>,
    req_body: web::Json<PostUserReqBody>,
) -> impl Responder {
    let db = pool.get().expect("couldn't get db connection from pool");

    let new_user = match req_body.0 {
        PostUserReqBody {
            username,
            email,
            password,
        } => models::UserForm {
            username,
            email,
            password,
        },
    };

    let insert_result = web::block(move || -> Result<(), diesel::result::Error> {
        use crate::schema::users::dsl::*;
        insert_into(users).values(&new_user).execute(&db)?;
        Ok(())
    })
    .await;

    match insert_result {
        Ok(_) => HttpResponse::Created().finish(),
        Err(err) => {
            eprint!("{}", err);
            match err {
                BlockingError::Error(DatabaseError(
                    DatabaseErrorKind::UniqueViolation,
                    err_info,
                )) => {
                    let contraint = err_info.message();
                    let mut title = contraint.to_string();
                    if contraint.contains("username") {
                        title = "Specified username already exists".into();
                    } else if contraint.contains("email") {
                        title = "Specified email already exists".into();
                    }
                    let errors = vec![JsonApiError { title }];
                    HttpResponse::Conflict().json(ErrorResponseBody { errors })
                }
                _ => HttpResponse::InternalServerError().finish(),
            }
        }
    }
}
