use crate::{
    constants::{
        EMAIL, ERROR_MESSAGE_EMAIL_UNIQUE_VIOLATION, ERROR_MESSAGE_USERNAME_UNIQUE_VIOLATION,
        TITLE, USERNAME,
    },
    models::{User, UserInsertTO},
    services::auth,
    types::DbPool,
    views::{ErrorResBody, SuccessfulResBody},
};
use actix_threadpool::BlockingError;
use actix_web::{get, post, web, HttpResponse, Responder};
use actix_web_grants::proc_macro::has_permissions;
use diesel::result::{DatabaseErrorKind, Error::DatabaseError};
use serde::Deserialize;
use serde_json::json;
use validator::Validate;

#[get("/")]
#[has_permissions("BASIC")]
pub async fn get_users(pool: web::Data<DbPool>) -> impl Responder {
    let connection = pool.get().unwrap();
    let users_result = web::block(move || User::get_all(&connection)).await;

    match users_result {
        Ok(users) => HttpResponse::Ok().json(SuccessfulResBody { data: users }),
        Err(err) => {
            eprint!("{}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[derive(Deserialize, Validate)]
pub struct PostUserReqBody {
    #[validate(length(min = 1, message = "Username too short"))]
    username: String,
    #[validate(
        length(min = 1, message = "Email too short"),
        email(message = "Email incorrect format")
    )]
    email: String,
    #[validate(length(min = 1, message = "Password too short"))]
    password: String,
}

#[post("/")]
pub async fn post_user(
    pool: web::Data<DbPool>,
    req_body: web::Json<PostUserReqBody>,
) -> impl Responder {
    if let Err(errors) = req_body.validate() {
        return HttpResponse::BadRequest().json(ErrorResBody::from(errors));
    }

    let password = match auth::hash(&req_body.password) {
        Ok(val) => val,
        Err(err) => {
            eprint!("{}", err);
            return HttpResponse::InternalServerError().finish();
        }
    };

    let new_user = match req_body.into_inner() {
        PostUserReqBody {
            username, email, ..
        } => UserInsertTO {
            username,
            email,
            password,
        },
    };

    let connection = pool.get().unwrap();
    let insert_result = web::block(move || User::insert(&new_user, &connection)).await;

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
                    if contraint.contains(USERNAME) {
                        title = ERROR_MESSAGE_USERNAME_UNIQUE_VIOLATION.into();
                    } else if contraint.contains(EMAIL) {
                        title = ERROR_MESSAGE_EMAIL_UNIQUE_VIOLATION.into();
                    }
                    let errors = vec![json!({ TITLE: title })];
                    HttpResponse::Conflict().json(ErrorResBody { errors })
                }
                _ => HttpResponse::InternalServerError().finish(),
            }
        }
    }
}
