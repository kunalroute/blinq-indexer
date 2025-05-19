use actix_web::{App, HttpServer};

mod routes;
mod handlers;
mod models;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        App::new()
            .configure(crate::routes::configure_routes)
    })
    .bind("127.0.0.1:8080")?;
    println!("🚀 Server started at http://127.0.0.1:8080");
    server.run().await
}
