use actix_web::{HttpResponse, Responder};
use crate::models::Order;

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().body("Service is running")
}

pub async fn get_orders() -> impl Responder {
    HttpResponse::Ok().json(vec![
        Order {
            id: 1,
            status: "completed".to_string(),
            source: "1".to_string(),
            destination: "137".to_string(),
        },
        Order {
            id: 2,
            status: "completed".to_string(),
            source: "56".to_string(),
            destination: "137".to_string(),
        },
    ])
}