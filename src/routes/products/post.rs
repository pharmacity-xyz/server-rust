use crate::{request::RequestProduct, response::ServiceResponse};
use actix_web::{web, HttpResponse, ResponseError};
use sqlx::PgPool;
use stripe::{Client, CreateProduct, Product};
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

    let stripe_product = insert_product_to_stripe(&product).await?;

    insert_product_to_db(&stripe_product.id.as_str(), &product, pool).await?;

    res.data = product_id;
    res.success = true;

    Ok(HttpResponse::Ok().json(res))
}

async fn insert_product_to_db(
    product_id: &str,
    product: &web::Json<RequestProduct>,
    pool: web::Data<PgPool>,
) -> Result<(), PostProductError> {
    sqlx::query!(
        r#"
        INSERT INTO products (product_id, product_name, product_description, image_url, stock, price, category_id, featured)
        VALUES ($1, $2, $3, $4, $5, $6, $7, false)
        "#,
        product_id,
        product.product_name,
        product.product_description,
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

async fn insert_product_to_stripe(
    product: &web::Json<RequestProduct>,
) -> Result<Product, PostProductError> {
    let secret_key = std::env::var("STRIPE_SECRET_KEY").expect("Missing STRIPE_SECRET_KEY in env");
    let client = Client::new(secret_key);

    let product = {
        let mut create_product = CreateProduct::new(product.product_name.as_str());
        create_product.description = Some(product.product_description.as_str());
        create_product.images = Some(vec![product.image_url.clone()]);
        create_product.metadata = Some(
            [("async-stripe".to_string(), "true".to_string())]
                .iter()
                .cloned()
                .collect(),
        );
        Product::create(&client, create_product).await.unwrap()
    };

    Ok(product)
}
