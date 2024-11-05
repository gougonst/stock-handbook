use crate::handlers::auth_handler::*;
use actix_web::web;

pub fn auth_scope(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/login", web::post().to(login))
            .route("/logon", web::post().to(logon)),
    );
}
