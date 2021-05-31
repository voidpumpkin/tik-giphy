use crate::utils::auth::TokenClaims;
use actix_web::{dev::ServiceRequest, Error};
use actix_web_grants::permissions::AttachPermissions;
use actix_web_httpauth::extractors::bearer::BearerAuth;

pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
    if let Ok(claims) = TokenClaims::from_token(credentials.token()) {
        req.attach(claims.permissions);
    }
    Ok(req)
}

#[macro_export]
macro_rules! auth_middleware {
    () => {{
        use crate::middlewares;
        use actix_web::web;
        use actix_web_httpauth::middleware::HttpAuthentication;

        web::scope("").wrap(HttpAuthentication::bearer(middlewares::validator))
    }};
}
