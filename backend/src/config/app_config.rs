#[macro_export]
macro_rules! app_config {
    ($pool:expr) => {{
        use crate::controllers::{get_users, login, post_user};
        use actix_web::{
            middleware::{Logger, NormalizePath},
            web, App,
        };

        App::new()
            .data($pool)
            .wrap(Logger::default())
            .wrap(NormalizePath::default())
            .service(web::scope("/auth").service(login))
            .service(
                web::scope("/users")
                    .service(post_user)
                    .service(auth_middleware!().service(get_users)),
            )
    }};
}
