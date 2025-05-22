use actix_web::{get, web, HttpResponse, Responder};

#[get("/api/health")]
async fn get_health() -> impl Responder {
    HttpResponse::Ok().json("OK")
}