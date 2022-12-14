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
    // StripeInsertionError(StripeError),
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
    // let new_product = insert_product_to_stripe(&product).await?;
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

// async fn insert_product_to_stripe(
//     product: &web::Json<RequestProduct>,
// ) -> Result<Product, PostProductError> {
//     let secret_key = std::env::var("STRIPE_SECRET_KEY").expect("Missing STRIPE_SECRET_KEY in env");
//     let client = Client::new(secret_key);

//     let product = {
//         let mut create_product = CreateProduct::new(product.name.as_str());
//         create_product.description = Some(product.description.as_str());
//         create_product.images = Some(vec![product.image_url.clone()]);
//         create_product.metadata = Some(
//             [("async-stripe".to_string(), "true".to_string())]
//                 .iter()
//                 .cloned()
//                 .collect(),
//         );
//         Product::create(&client, create_product).await.unwrap()
//     };

//     Ok(product)
// }
