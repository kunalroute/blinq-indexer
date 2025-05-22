use std::env;

use actix_web::{App, HttpServer};
use modules::{health::handlers::get_health, orders::handlers::get_orders};

mod config;
mod models;
mod modules;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    
    dotenv::dotenv().ok(); // load env
    env_logger::init(); // load RUST_LOG for logging

    // Read environment variables
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    HttpServer::new(|| {
        App::new()
            .configure(crate::config::config_routes) // Call the config function to set up routes
    })
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await
}
