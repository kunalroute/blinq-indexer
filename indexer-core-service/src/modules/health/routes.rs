use actix_web::web;

use super::handlers::get_health;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api").service(get_health));
}
