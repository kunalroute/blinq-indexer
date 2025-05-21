use actix_web::{get, HttpResponse, Responder, web::Query};

use crate::modules::orders::services as orders_service;

#[derive(serde::Deserialize)]
pub struct GetOrdersParams {
    pub hash: String,
}

#[get("/orders")]
async fn get_orders(params: Query<GetOrdersParams>) -> impl Responder {
    let hash = &params.hash;
    match orders_service::get_orders(hash).await {
        Ok(orders) => HttpResponse::Ok().json(orders),
        Err(e) => {
            eprintln!("Error fetching orders: {}", e);
            HttpResponse::InternalServerError().body("Failed to fetch orders")
        }
    }
}