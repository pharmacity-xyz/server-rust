use actix_web::{web, HttpResponse, ResponseError};
use sqlx::PgPool;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Product {
    id: String,
    name: String,
    description: String,
    image_url: String,
    stock: i32,
    price: i32,
    category_id: uuid::Uuid,
    featured: bool,
}

pub async fn update_product(
    product: web::Json<Product>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, UpdateProductError> {
    sqlx::query!(
        r#"
        UPDATE products 
        SET name = $1, description = $2, image_url = $3, stock = $4, price = $5, category_id = $6, featured = $7
        WHERE id = $8
        "#,
        product.name,
        product.description,
        product.image_url,
        product.stock,
        product.price,
        product.category_id,
        product.featured,
        product.id,
    )
    .execute(pool.get_ref())
    .await
    .map_err(UpdateProductError)?;
    Ok(HttpResponse::Ok().finish())
}

#[derive(Debug)]
pub struct UpdateProductError(sqlx::Error);

impl ResponseError for UpdateProductError {}

impl std::fmt::Display for UpdateProductError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to update products.")
    }
}
