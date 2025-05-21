use actix_web::web;

use crate::modules::health::routes as health_routes;
use crate::modules::orders::routes as order_routes;

// call individual modules route config
pub fn config_routes(cfg: &mut web::ServiceConfig) {
   health_routes::config(cfg);
   order_routes::config(cfg);
}
