use crate::{
    models,
    utils::{auth, ErrorResBody, JsonApiError, SuccessfulResBody},
    DbPool,
};
use actix_web::{post, web, HttpResponse, Responder, Scope};
use diesel::{result::QueryResult, ExpressionMethods, QueryDsl, RunQueryDsl};
use serde::Deserialize;
use serde_json::json;
use validator::Validate;

pub fn auth() -> Scope {
    web::scope("/auth").service(login)
}

#[derive(Deserialize, Validate)]
struct LoginReqBody {
    #[validate(
        length(min = 1, message = "Email too short"),
        email(message = "Email incorrect format")
    )]
    email: String,
    #[validate(length(min = 1, message = "Password too short"))]
    password: String,
}

#[post("/login/")]
async fn login(pool: web::Data<DbPool>, req_body: web::Json<LoginReqBody>) -> impl Responder {
    if let Err(errs) = req_body.validate() {
        let errors: Vec<JsonApiError> = errs
            .field_errors()
            .iter()
            .map(|field_err| {
                field_err.1.iter().map(|validation_err| {
                    let title = match validation_err.clone().message {
                        Some(message) => message.to_string(),
                        None => format!("{}", validation_err),
                    };
                    JsonApiError { title }
                })
            })
            .flatten()
            .collect();
        return HttpResponse::Unauthorized().json(ErrorResBody { errors });
    }

    let LoginReqBody {
        email: req_email,
        password: req_pass,
    } = req_body.into_inner();

    let db = pool.get().expect("couldn't get db connection from pool");

    let user_result = web::block(move || -> QueryResult<models::User> {
        use crate::schema::users::dsl::*;
        let user = users
            .filter(email.eq(&req_email))
            .first::<models::User>(&db)?;
        Ok(user)
    })
    .await;

    let models::User { id, password, .. } = match user_result {
        Ok(user) => user,
        Err(err) => {
            eprint!("{}", err);
            return HttpResponse::Unauthorized()
                .json(ErrorResBody::new_single_error("User not found"));
        }
    };

    match auth::verify(&req_pass, &password) {
        Ok(is_maching) if !is_maching => {
            return HttpResponse::Unauthorized()
                .json(ErrorResBody::new_single_error("Incorrect password"));
        }
        Err(err) => {
            eprint!("{}", err);
            return HttpResponse::InternalServerError().finish();
        }
        _ => {}
    }

    match auth::generate_token(id) {
        Ok((token, expires_in)) => HttpResponse::Ok().json(SuccessfulResBody {
            data: json!({
                "access_token": token,
                "token_type": "bearer",
                "expires_in": expires_in,
            }),
        }),
        Err(err) => {
            eprint!("{}", err);
            return HttpResponse::InternalServerError().finish();
        }
    }
}
