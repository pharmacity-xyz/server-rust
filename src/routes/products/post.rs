use crate::{request::RequestProduct, response::ServiceResponse};
use actix_web::{web, HttpResponse, ResponseError};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug)]
pub enum PostProductError {
    DatabaseError(sqlx::Error),
}

impl ResponseError for PostProductError {}

impl std::fmt::Display for PostProductError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to post products.")
    }
}

pub async fn post_product(
    product: web::Json<RequestProduct>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, PostProductError> {
    let mut res = ServiceResponse::new(Uuid::default());

    let product_id = Uuid::new_v4();

    insert_product_to_db(&product_id, &product, pool).await?;

    res.data = product_id;
    res.success = true;

    Ok(HttpResponse::Ok().json(res))
}

async fn insert_product_to_db(
    product_id: &Uuid,
    product: &web::Json<RequestProduct>,
    pool: web::Data<PgPool>,
) -> Result<(), PostProductError> {
    sqlx::query!(
        r#"
        INSERT INTO products (product_id, product_name, product_description, image_url, stock, price, category_id, featured)
        VALUES ($1, $2, $3, $4, $5, $6, $7, false)
        "#,
        product_id,
        product.name,
        product.description,
        product.image_url,
        product.stock,
        product.price,
        product.category_id,
    )
    .execute(pool.get_ref())
    .await
    .map_err(PostProductError::DatabaseError)?;

    Ok(())
}
