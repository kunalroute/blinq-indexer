use actix_web::web;

pub mod health;
pub mod order;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(health::health_routes())
        .service(order::order_routes());
}