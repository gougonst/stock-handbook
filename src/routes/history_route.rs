use crate::handlers::history_handler::*;
use actix_web::web;

pub fn history_scope(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/history").route("/list", web::get().to(list_history)));
}
