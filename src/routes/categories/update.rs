use actix_web::{web, HttpResponse, ResponseError};
use sqlx::PgPool;
use uuid::Uuid;

use crate::response::ServiceResponse;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UpdateCategory {
    category_id: uuid::Uuid,
    name: String,
}

pub async fn update_category(
    category: web::Json<UpdateCategory>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, UpdateCategoryError> {
    let mut res = ServiceResponse::new(Uuid::default());

    sqlx::query!(
        r#"
        UPDATE categories 
        SET name = $1
        WHERE category_id = $2
        "#,
        category.name,
        category.category_id
    )
    .execute(pool.get_ref())
    .await
    .map_err(UpdateCategoryError)?;

    res.data = category.category_id;
    res.success = true;

    Ok(HttpResponse::Ok().json(res))
}

#[derive(Debug)]
pub struct UpdateCategoryError(sqlx::Error);

impl ResponseError for UpdateCategoryError {}

impl std::fmt::Display for UpdateCategoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to update categories.")
    }
}
