use actix_web::{web, HttpResponse, ResponseError};
use sqlx::PgPool;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct User {
    id: uuid::Uuid,
    email: String,
    password: String,
    first_name: String,
    last_name: String,
    city: String,
    country: String,
    company_name: String,
    role: String,
}

pub async fn update_user(
    user: web::Json<User>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, UpdateUserError> {
    sqlx::query!(
        r#"
        UPDATE users
        SET email = $1, first_name = $2, last_name = $3, city = $4, country = $5, company_name = $6
        WHERE id = $7
        "#,
        user.email,
        user.first_name,
        user.last_name,
        user.city,
        user.country,
        user.company_name,
        user.id
    )
    .execute(pool.get_ref())
    .await
    .map_err(UpdateUserError)?;
    Ok(HttpResponse::Ok().finish())
}

#[derive(Debug)]
pub struct UpdateUserError(sqlx::Error);

impl ResponseError for UpdateUserError {}

impl std::fmt::Display for UpdateUserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to update user.")
    }
}
