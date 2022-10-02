use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct User {
    name: String,
    address: String,
    phonenumber: String,
    email: String,
    password: String,
}

pub async fn post_user(user: web::Json<User>, pool: web::Data<PgPool>) -> HttpResponse {
    log::info!("Saving new user details in the database");
    match sqlx::query!(
        r#"
        INSERT INTO users (id, name, address, phonenumber, email, password)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        Uuid::new_v4(),
        user.name,
        user.address,
        user.phonenumber,
        user.email,
        user.password
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => {
            log::info!("New user details have been saved");
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            log::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
