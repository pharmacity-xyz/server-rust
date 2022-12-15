use crate::{response::ServiceResponse, types::product::Product};
use actix_web::{web, HttpResponse, ResponseError};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug)]
pub enum UpdateProductError {
    DatabaseError(sqlx::Error),
}

impl ResponseError for UpdateProductError {}

impl std::fmt::Display for UpdateProductError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to update products.")
    }
}

pub async fn update_product(
    product: web::Json<Product>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, UpdateProductError> {
    let mut res = ServiceResponse::new(Uuid::default());

    update_product_for_db(&product, pool).await?;

    res.data = product.product_id;
    res.success = true;

    Ok(HttpResponse::Ok().json(res))
}

async fn update_product_for_db(
    product: &web::Json<Product>,
    pool: web::Data<PgPool>,
) -> Result<(), UpdateProductError> {
    sqlx::query!(
        r#"
        UPDATE products 
        SET product_name = $1, product_description = $2, image_url = $3, stock = $4, price = $5, category_id = $6, featured = $7
        WHERE product_id = $8
        "#,
        product.name,
        product.description,
        product.image_url,
        product.stock,
        product.price,
        product.category_id,
        product.featured,
        product.product_id,
    )
    .execute(pool.get_ref())
    .await
    .map_err(UpdateProductError::DatabaseError)?;

    Ok(())
}
