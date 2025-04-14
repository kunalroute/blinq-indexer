use actix_web::{HttpResponse, Responder};

use crate::utils;

pub async fn get_order(hash: str) -> impl Responder {
    HttpResponse::Ok().body(utils::fetch_cctp_transaction_status(hash).await.unwrap());
}

pub async fn create_order() -> impl Responder {
    HttpResponse::Ok().body("Order created")
}