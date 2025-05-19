use actix_web::{get, web, HttpResponse, Scope};
use crate::handlers::order::{create_order, get_order, OrderQuery};

// pub fn order_routes() -> Scope {
//     web::scope("/orders")
//         .route("", web::get().to(get_order))
//         .route("", web::post().to(create_order))
// }

#[get("/orders")]
pub async fn order_routes(query: web::Query<OrderQuery>) -> HttpResponse {
    let hash = &query.hash;

    // Simulated logic: fetch order based on hash
    // In a real app, you'd query a database or service
    let order_details = format!("Fetched order with hash: {}", hash);

    HttpResponse::Ok().body(order_details)
}