use actix_web::{web, HttpResponse, ResponseError};
use sqlx::PgPool;
use uuid::Uuid;

use crate::response::ServiceResponse;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct PostCategory {
    name: String,
}

pub async fn post_category(
    category: web::Json<PostCategory>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, PostCategoryError> {
    let mut res = ServiceResponse::new(Uuid::default());

    let category_id = uuid::Uuid::new_v4();

    sqlx::query!(
        r#"
        INSERT INTO categories (category_id, name)
        VALUES ($1, $2)
        "#,
        category_id,
        category.name
    )
    .execute(pool.get_ref())
    .await
    .map_err(PostCategoryError)?;

    res.data = category_id.clone();
    res.success = true;

    Ok(HttpResponse::Ok().json(res))
}

#[derive(Debug)]
pub struct PostCategoryError(sqlx::Error);

impl ResponseError for PostCategoryError {}

impl std::fmt::Display for PostCategoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to post categories.")
    }
}
