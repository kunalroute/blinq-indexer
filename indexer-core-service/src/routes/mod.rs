use actix_web::web;

pub fn setup_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/orders", web::get().to(crate::handlers::get_orders));
}
