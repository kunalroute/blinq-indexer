use actix_web::{App, HttpServer};

fn main() {
    HttpServer::new(|| {
        App::new()
            .configure(crate::routes::setup_routes)
    })
    .bind("127.0.0.1:8080")
    .expect("Cannot bind to port 8080")
    .run()
    .unwrap();
}
