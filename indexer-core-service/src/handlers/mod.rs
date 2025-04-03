use actix_web::{HttpResponse, Responder};

pub fn handle_request() {
    println!("Handling request...");
}

pub async fn get_orders() -> impl Responder {
    HttpResponse::Ok().json(vec![
        {"id": 1, "item": "Book", "quantity": 2},
        {"id": 2, "item": "Laptop", "quantity": 1},
    ])
}
