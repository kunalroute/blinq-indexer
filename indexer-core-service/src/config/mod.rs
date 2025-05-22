use actix_web::web;

use crate::modules::health::handlers::get_health;
use crate::modules::orders::handlers::get_orders;

// call individual modules route config
pub fn config_routes(cfg: &mut web::ServiceConfig) {
   cfg
      .service(get_health)
      .service(get_orders);
}
