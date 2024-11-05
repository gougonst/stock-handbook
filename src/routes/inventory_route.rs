use crate::handlers::inventory_handler::*;
use actix_web::web;

pub fn inventory_scope(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/inventory")
            .route("/list", web::get().to(list_inventory_summary))
            .route("/add", web::post().to(buy_stock))
            .route("/delete", web::post().to(sell_stock)),
    );
}
