use actix_web::{get, post, web, HttpResponse, Responder, Scope};

pub fn users() -> Scope {
    web::scope("/users")
        .service(get_users)
        .service(get_me)
        .service(post_user)
}

#[get("/")]
async fn get_users() -> impl Responder {
    HttpResponse::Ok().body("I am a user")
}

#[get("/me")]
async fn get_me() -> impl Responder {
    HttpResponse::Ok().body("I am a you")
}

#[post("/")]
async fn post_user(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(format!("You posted user: {}", req_body))
}
