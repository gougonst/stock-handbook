use crate::handlers::stocks::*;
use actix_web::web;

pub fn stock_scope(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/stock")
            .route("/list", web::get().to(list_stocks))
            .route("/add", web::post().to(add_stock))
    );
}
