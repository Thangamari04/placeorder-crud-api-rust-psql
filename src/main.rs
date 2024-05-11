use actix_web::{web, App, HttpResponse, HttpServer, Responder, HttpRequest};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPool;
use sqlx::Row;
use serde_json::json;
use validator::{Validate}; 
use validator_derive::Validate; 

#[derive(Debug, Deserialize, Serialize, Validate)]
struct PersonalOrder {
    #[validate(length(min = 2))]
    customer_name: String,
    #[validate(email)]
    email: String,
    #[validate(length(min = 10))]
    address: String,
    #[validate(length(min = 6))]
    pincode: String,
    #[validate(length(min = 2))]
    state: String,
    #[validate(length(min = 2))]
    city: String,
    #[validate(length(min = 10))]
    phone_number: String,
}

async fn place_order(order: web::Json<PersonalOrder>, pool: web::Data<PgPool>) -> impl Responder {

    if let Err(errors) = order.validate() {
        return HttpResponse::BadRequest().json(errors);
    }

    let result = sqlx::query(
        r#"INSERT INTO personal_orders (customer_name, email, address, pincode, state, city, phone_number)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id"#,
    )
    .bind(&order.customer_name)
    .bind(&order.email)
    .bind(&order.address)
    .bind(&order.pincode)
    .bind(&order.state)
    .bind(&order.city)
    .bind(&order.phone_number)
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(row) => {
            let id: i32 = row.get("id");
            HttpResponse::Created().json(json!({ "message": "User data inserted successfully" }))
        }
        Err(_) => HttpResponse::InternalServerError().body("Failed to insert user data"),
    }
}

async fn update_order(path: web::Path<(i32,)>, order: web::Json<PersonalOrder>, pool: web::Data<PgPool>) -> impl Responder {
    let id = path.0; // Extracting the i32 value from web::Path<i32>

    if let Err(errors) = order.validate() {
        return HttpResponse::BadRequest().json(errors);
    }

    let result = sqlx::query(
        r#"UPDATE personal_orders 
        SET customer_name = $1, email = $2, address = $3, pincode = $4, state = $5, city = $6, phone_number = $7
        WHERE id = $8"#,
    )
    .bind(&order.customer_name)
    .bind(&order.email)
    .bind(&order.address)
    .bind(&order.pincode)
    .bind(&order.state)
    .bind(&order.city)
    .bind(&order.phone_number)
    .bind(id) 
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("User data updated successfully"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to update user data"),
    }
}


async fn delete_order(path: web::Path<(i32,)>, pool: web::Data<PgPool>) -> impl Responder {
    let id = path.0; 

    let result = sqlx::query(
        "DELETE FROM personal_orders WHERE id = $1",
    )
    .bind(id) 
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().body("User data deleted successfully"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to delete user data"),
    }
    
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("postgres://postgres:postgres@localhost:5432/postgres");
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(web::resource("/place_order").route(web::post().to(place_order)))
            .service(web::resource("/order/{id}").route(web::put().to(update_order)))
            .service(web::resource("/order/{id}").route(web::delete().to(delete_order)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
