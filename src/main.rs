mod handlers;

use actix_web::{web, App, HttpServer};
//use crate::PgPool;
use sqlx::postgres::PgPool;
//use sqlx::PgPool;
//use handlers::configure_routes;




mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the database pool
    let pool = PgPool::connect("postgres://postgres:postgres@localhost:5432/postgres").await.unwrap();

    // Start Actix Web server
    HttpServer::new(move || {
        App::new()
            // Pass the database pool to the handlers
            .data(pool.clone())
            // Configure routes
            .configure(handlers::configure_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
