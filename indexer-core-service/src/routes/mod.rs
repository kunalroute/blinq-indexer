use actix_web::{web};
use crate::handlers::health_check;
use crate::handlers::get_orders;

pub fn setup_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check));
    cfg.route("/orders", web::get().to(get_orders));
}

