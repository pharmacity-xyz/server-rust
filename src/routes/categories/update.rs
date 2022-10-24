use actix_web::{web, HttpResponse, ResponseError};
use sqlx::PgPool;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Category {
    id: uuid::Uuid,
    name: String,
}

pub async fn update_category(
    category: web::Json<Category>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, UpdateCategoryError> {
    sqlx::query!(
        r#"
        UPDATE categories 
        SET name = $1
        WHERE id = $2
        "#,
        category.name,
        category.id
    )
    .execute(pool.get_ref())
    .await
    .map_err(UpdateCategoryError)?;
    Ok(HttpResponse::Ok().finish())
}

#[derive(Debug)]
pub struct UpdateCategoryError(sqlx::Error);

impl ResponseError for UpdateCategoryError {}

impl std::fmt::Display for UpdateCategoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to update categories.")
    }
}
