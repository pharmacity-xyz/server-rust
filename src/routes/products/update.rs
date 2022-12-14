use crate::types::product::Product;
use actix_web::{web, HttpResponse, ResponseError};
use sqlx::PgPool;

#[derive(Debug)]
pub enum UpdateProductError {
    DatabaseError(sqlx::Error),
    // StripeUpdateError(StripeError),
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
    // let updated_product = update_product_for_stripe(&product).await?;
    update_product_for_db(&product, pool).await?;
    Ok(HttpResponse::Ok().json(""))
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

// async fn update_product_for_stripe(
//     product: &web::Json<RequestProduct>,
// ) -> Result<Product, UpdateProductError> {
//     let secret_key = std::env::var("STRIPE_SECRET_KEY").expect("Missing STRIPE_SECRET_KEY in env");
//     let client = Client::new(secret_key);

//     let products = Product::list(
//         &client,
//         ListProducts {
//             ids: Some(vec![product.id.clone()]),
//             ..Default::default()
//         },
//     )
//     .await
//     .expect("Fail to fetch product");

//     let product = Product::update(
//         &client,
//         &products.data[0].id,
//         UpdateProduct {
//             name: Some(product.name.as_str()),
//             images: Some(vec![product.image_url.clone()]),
//             ..Default::default()
//         },
//     )
//     .await
//     .expect("Fail to update product");

//     Ok(product)
// }
