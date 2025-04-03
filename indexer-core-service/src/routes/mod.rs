use actix_web::{web, HttpResponse};

pub fn setup_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check));
    // Uncomment and implement the following line when the `get_orders` handler is ready
    // cfg.route("/orders", web::get().to(crate::handlers::get_orders));
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().body("Service is running")
}

