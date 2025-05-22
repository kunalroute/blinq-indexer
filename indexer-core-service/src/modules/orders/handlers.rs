use actix_web::{get, web::{self, Query}, HttpResponse, Responder};

use crate::modules::orders::services as orders_service;

#[derive(serde::Deserialize)]
pub struct OrderRequestDTO {
    pub hash: String,
}

#[get("/api/orders")]
async fn get_orders(order_request_dto: web::Query<OrderRequestDTO>) -> impl Responder {
    let hash = &order_request_dto.hash;
    match orders_service::get_orders(hash).await {
        Ok(orders) => HttpResponse::Ok().json(orders),
        Err(e) => {
            eprintln!("Error fetching orders: {}", e);
            HttpResponse::InternalServerError().body("Failed to fetch orders")
        }
    }
}