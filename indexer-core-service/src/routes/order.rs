use actix_web::{web, Scope};
use crate::handlers::order::{get_order, create_order};

pub fn order_routes() -> Scope {
    web::scope("/orders")
        .route("", web::get().to(|req| {
            let hash: String = req.query_string().to_string(); // Extract hash from query string
            get_order(hash)
        }))
        .route("", web::post().to(create_order))
}