use crate::{
    constants::{
        ACCESS_TOKEN, ERROR_MESSAGE_INCORRECT_PASSWORD, ERROR_MESSAGE_USER_NOT_FOUND, EXPIRES_IN,
        JWT_TOKEN_TYPE_BEARER, TOKEN_TYPE, USER_ROLE_BASIC,
    },
    models::User,
    services::auth,
    types::DbPool,
    views::{ErrorResBody, SuccessfulResBody},
};
use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::json;
use validator::Validate;

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
    if let Err(errors) = req_body.validate() {
        return HttpResponse::BadRequest().json(ErrorResBody::from(errors));
    }

    let LoginReqBody {
        email: req_email,
        password: req_pass,
    } = req_body.into_inner();

    let connection = pool.get().unwrap();
    let user_result = web::block(move || User::get_by_email(&req_email, &connection)).await;

    let User { id, password, .. } = match user_result {
        Ok(user) => user,
        Err(err) => {
            eprint!("{}", err);
            return HttpResponse::Unauthorized()
                .json(ErrorResBody::new_single_error(ERROR_MESSAGE_USER_NOT_FOUND));
        }
    };

    match auth::verify(&req_pass, &password) {
        Ok(is_maching) if !is_maching => {
            return HttpResponse::Unauthorized().json(ErrorResBody::new_single_error(
                ERROR_MESSAGE_INCORRECT_PASSWORD,
            ));
        }
        Err(err) => {
            eprint!("{}", err);
            return HttpResponse::InternalServerError().finish();
        }
        _ => {}
    }

    match auth::generate_token(id, vec![USER_ROLE_BASIC.into()]) {
        Ok((token, expires_in)) => HttpResponse::Ok().json(SuccessfulResBody {
            data: json!({
                ACCESS_TOKEN: token,
                TOKEN_TYPE: JWT_TOKEN_TYPE_BEARER,
                EXPIRES_IN: expires_in,
            }),
        }),
        Err(err) => {
            eprint!("{}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}
