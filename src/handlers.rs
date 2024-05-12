use actix_web::{web, App, HttpResponse};
use sqlx::postgres::PgPool;
use validator::Validate;
use crate::models::PlaceOrder;
//use serde_json::json;
use serde::Deserialize;
use actix_web::HttpServer;



pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api")
        .route("/hello", web::get().to(hello))
        .route("/place_order", web::post().to(place_order))
        .route("/get_order/{id}", web::get().to(get_order))
        .route("/update_order/{id}", web::put().to(update_order))
        .route("/delete_order/{id}", web::delete().to(delete_order))
    );
}

async fn hello() -> HttpResponse {
    HttpResponse::Ok().body("Hello, World!")
}

// POST handler to validate and place order
pub async fn place_order(
    place_order: web::Json<PlaceOrder>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let order_data = place_order.into_inner();

    // Validate order data
    if let Err(errors) = order_data.validate() {
        return HttpResponse::BadRequest().json(errors);
    }

    // TODO: Save order details to the database using SQLx

    HttpResponse::Ok().body("Order placed successfully")
}

// GET handler to fetch order details by ID
pub async fn get_order(
    order_id: web::Path<i32>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let order_id = order_id.into_inner();

    // TODO: Fetch order details from the database using SQLx

    // Placeholder response
    HttpResponse::Ok().body(format!("Order with ID {} fetched successfully", order_id))
}

// PUT handler to update order details by ID
pub async fn update_order(
    order_id: web::Path<i32>,
    place_order: web::Json<PlaceOrder>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let order_id = order_id.into_inner();
    let order_data = place_order.into_inner();

    // Validate order data
    if let Err(errors) = order_data.validate() {
        return HttpResponse::BadRequest().json(errors);
    }

    // TODO: Update order details in the database using SQLx

    HttpResponse::Ok().body(format!("Order with ID {} updated successfully", order_id))
}

// DELETE handler to delete order details by ID

pub async fn delete_order(
    order_id: web::Path<i32>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let order_id = order_id.into_inner();

    // TODO: Delete order details from the database using SQLx

    HttpResponse::Ok().body(format!("Order with ID {} deleted successfully", order_id))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the database pool
    let pool = PgPool::connect("postgres://postgres:postgres@localhost:5432/postgres").await.unwrap();

    // Start Actix Web server
    HttpServer::new(move || {
        App::new()
            // Pass the database pool to the handlers
            .data(pool.clone())
            
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}