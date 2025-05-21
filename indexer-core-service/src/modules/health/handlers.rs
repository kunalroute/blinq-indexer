use actix_web::{get, HttpResponse, Responder};

#[get("/health")]
async fn get_health() -> impl Responder {
    HttpResponse::Ok().json("OK")
}