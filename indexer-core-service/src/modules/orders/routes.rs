use actix_web::web;

use super::handlers::get_orders;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api").service(get_orders));
}
