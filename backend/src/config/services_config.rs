#[macro_export]
macro_rules! services_config {
    () => {{
        use crate::resources;
        use actix_web::web;

        web::scope("")
            .service(web::scope("/auth").service(resources::auth::login))
            .service(
                web::scope("/users")
                    .service(resources::users::post_user)
                    .service(auth_middleware!().service(resources::users::get_users)),
            )
    }};
}
