use actix_web::{web, HttpResponse, ResponseError};
use sqlx::PgPool;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Category {
    name: String,
}

pub async fn post_category(
    category: web::Json<Category>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, PostCategoryError> {
    sqlx::query!(
        r#"
        INSERT INTO categories (id, name)
        VALUES ($1, $2)
        "#,
        uuid::Uuid::new_v4(),
        category.name
    )
    .execute(pool.get_ref())
    .await
    .map_err(PostCategoryError)?;
    Ok(HttpResponse::Ok().finish())
}

#[derive(Debug)]
pub struct PostCategoryError(sqlx::Error);

impl ResponseError for PostCategoryError {}

impl std::fmt::Display for PostCategoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to post categories.")
    }
}
