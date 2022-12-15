use actix_web::{web, HttpResponse, ResponseError};
use bigdecimal::BigDecimal;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct RequestProduct {
    name: String,
    description: String,
    image_url: String,
    stock: i32,
    price: BigDecimal,
    category_id: uuid::Uuid,
}

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
    insert_product_to_db(Uuid::new_v4(), &product, pool).await?;
    Ok(HttpResponse::Ok().json(""))
}

async fn insert_product_to_db(
    product_id: Uuid,
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
