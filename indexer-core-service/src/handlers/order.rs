use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use crate::utils; // add this import

#[derive(Deserialize)]
pub struct OrderQuery {
    pub hash: String,
}

pub async fn get_order(query: web::Query<OrderQuery>) -> HttpResponse {
    println!("get_order");
    let hash = &query.hash;
    // Call fetch_transaction_status from utils
    let status = utils::fetch_transaction_status(hash).await;
    // For demonstration, just return the status as a string (you may want to serialize properly)
    HttpResponse::Ok().body(format!("Order status: {:?}", status))
}

pub async fn create_order() -> impl Responder {
    HttpResponse::Ok().body("Order created")
}