use actix_web::{web, HttpResponse, ResponseError};
use sqlx::PgPool;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Product {
    name: String,
    description: String,
    image_url: String,
    stock: i32,
    price: i32,
    category_id: uuid::Uuid,
}

pub async fn post_product(
    product: web::Json<Product>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, PostProductError> {
    sqlx::query!(
        r#"
        INSERT INTO products (id, name, description, image_url, stock, price, category_id, featured)
        VALUES ($1, $2, $3, $4, $5, $6, $7, false)
        "#,
        uuid::Uuid::new_v4(),
        product.name,
        product.description,
        product.image_url,
        product.stock,
        product.price,
        product.category_id,
    )
    .execute(pool.get_ref())
    .await
    .map_err(PostProductError)?;
    Ok(HttpResponse::Ok().finish())
}

#[derive(Debug)]
pub struct PostProductError(sqlx::Error);

impl ResponseError for PostProductError {}

impl std::fmt::Display for PostProductError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to post products.")
    }
}
