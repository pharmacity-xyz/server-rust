use actix_web::{web, HttpResponse, ResponseError};
use sqlx::PgPool;

use crate::response::ServiceResponse;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Category {
    id: uuid::Uuid,
    name: String,
}

pub async fn get_categories(pool: web::Data<PgPool>) -> Result<HttpResponse, GetCategoriesError> {
    let mut res = ServiceResponse::new(Vec::<Category>::new());

    let categories = sqlx::query!(
        r#"
        SELECT * FROM categories
        "#
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(GetCategoriesError)?;

    let mut vec_categories = vec![];

    for category in categories.into_iter() {
        let temp_category = Category {
            id: category.category_id,
            name: category.name,
        };

        vec_categories.push(temp_category);
    }

    res.data = vec_categories;
    res.success = true;

    Ok(HttpResponse::Ok().json(res))
}

#[derive(Debug)]
pub struct GetCategoriesError(sqlx::Error);

impl ResponseError for GetCategoriesError {}

impl std::fmt::Display for GetCategoriesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to get all categories.")
    }
}
