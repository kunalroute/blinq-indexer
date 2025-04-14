use actix_web::{web, Scope};
use crate::handlers::health::get_health;

pub fn health_routes() -> Scope {
    web::scope("/health")
        .route("", web::get().to(get_health))
}