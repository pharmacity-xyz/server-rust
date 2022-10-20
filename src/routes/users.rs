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

#[tracing::instrument(
    name = "Addming a new user",
    skip(user, pool),
    fields(
        request_id = %Uuid::new_v4(),
        user_email = %user.email,
        user_first_name = %user.first_name
    )
)]
pub async fn post_user(user: web::Json<User>, pool: web::Data<PgPool>) -> HttpResponse {
    match insert_user(&pool, &user).await {
        Ok(_) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(name = "Saving new user details in the database", skip(user, pool))]
pub async fn insert_user(pool: &PgPool, user: &web::Json<User>) -> Result<(), sqlx::Error> {
    let new_id = Uuid::new_v4();
    sqlx::query!(
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
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}
