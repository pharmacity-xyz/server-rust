use actix_web::{web, HttpResponse, ResponseError};
use sqlx::PgPool;
use stripe::{Client, ListProducts, Product, StripeError, UpdateProduct};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct RequestProduct {
    id: String,
    name: String,
    description: String,
    image_url: String,
    stock: i32,
    price: i32,
    category_id: uuid::Uuid,
    featured: bool,
}

#[derive(Debug)]
pub enum UpdateProductError {
    DatabaseError(sqlx::Error),
    StripeUpdateError(StripeError),
}

impl ResponseError for UpdateProductError {}

impl std::fmt::Display for UpdateProductError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to update products.")
    }
}

pub async fn update_product(
    product: web::Json<RequestProduct>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, UpdateProductError> {
    Ok(HttpResponse::Ok().finish())
}

async fn update_product_for_stripe(
    product: &web::Json<RequestProduct>,
) -> Result<Product, UpdateProductError> {
    let secret_key = std::env::var("STRIPE_SECRET_KEY").expect("Missing STRIPE_SECRET_KEY in env");
    let client = Client::new(secret_key);

    let products = Product::list(
        &client,
        ListProducts {
            ids: Some(vec![product.id.clone()]),
            ..Default::default()
        },
    )
    .await
    .expect("Fail to fetch product");

    let product = Product::update(
        &client,
        &products.data[0].id,
        UpdateProduct {
            name: Some(product.name.as_str()),
            images: Some(vec![product.image_url.clone()]),
            ..Default::default()
        },
    )
    .await
    .expect("Fail to update product");

    Ok(product)
}
async fn update_product_for_db(
    product: &web::Json<RequestProduct>,
    pool: web::Data<PgPool>
) -> Result<(), UpdateProductError> {
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
    .map_err(UpdateProductError::DatabaseError)?;

    Ok(())
}
