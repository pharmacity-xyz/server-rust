use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct User {
    email: String,
    password: String,
    first_name: String,
    last_name: String,
    city: String,
    country: String,
    company_name: String,
}

pub async fn post_user(user: web::Json<User>, pool: web::Data<PgPool>) -> HttpResponse {
    log::info!("Saving new user details in the database");
    let new_id = Uuid::new_v4();
    match sqlx::query!(
        r#"
        INSERT INTO users (id, email, password, first_name, last_name, city, country, company_name, role)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#,
        new_id,
        user.email,
        user.password,
        user.first_name,
        user.last_name,
        user.city,
        user.country,
        user.company_name,
        "User".to_string()
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => {
            log::info!("New user details have been saved");
            HttpResponse::Ok().json(user)
            // HttpResponse::Ok().finish()
        }
        Err(e) => {
            log::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
