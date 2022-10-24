use actix_web::{web, HttpResponse, ResponseError};
use sqlx::PgPool;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Product {
    id: uuid::Uuid,
    name: String,
    description: String,
    image_url: String,
    stock: i32,
    price: i32,
    category_id: uuid::Uuid,
}

pub async fn get_all_products(
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, GetAllProductsError> {
    let products = sqlx::query!(
        r#"
        SELECT * FROM products
        "#
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(GetAllProductsError)?;

    let mut vec_products = vec![];

    for product in products.into_iter() {
        let temp_product = Product {
            id: product.id,
            name: product.name,
            description: product.description,
            image_url: product.image_url,
            stock: product.stock,
            price: product.price,
            category_id: product.category_id,
        };

        vec_products.push(temp_product);
    }

    Ok(HttpResponse::Ok().json(vec_products))
}

#[derive(Debug)]
pub struct GetAllProductsError(sqlx::Error);

impl ResponseError for GetAllProductsError {}

impl std::fmt::Display for GetAllProductsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to get all categories.")
    }
}

#[derive(serde::Deserialize)]
pub struct ProductId {
    id: uuid::Uuid,
}

pub async fn get_product_by_productid(
    pool: web::Data<PgPool>,
    product_id: web::Query<ProductId>,
) -> Result<HttpResponse, GetAllProductsError> {
    let product = sqlx::query!(
        r#"
        SELECT * FROM products
        WHERE id = $1
        "#,
        product_id.id
    )
    .fetch_one(pool.get_ref())
    .await
    .map_err(GetAllProductsError)?;

    let temp_product = Product {
        id: product.id,
        name: product.name,
        description: product.description,
        image_url: product.image_url,
        stock: product.stock,
        price: product.price,
        category_id: product.category_id,
    };

    Ok(HttpResponse::Ok().json(temp_product))
}

#[derive(serde::Deserialize)]
pub struct CategoryId {
    id: uuid::Uuid,
}

pub async fn get_product_by_categoryid(
    pool: web::Data<PgPool>,
    category_id: web::Query<CategoryId>,
) -> Result<HttpResponse, GetAllProductsError> {
    let products = sqlx::query!(
        r#"
        SELECT * FROM products
        WHERE category_id = $1
        "#,
        category_id.id
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(GetAllProductsError)?;

    let mut vec_products = vec![];

    for product in products.into_iter() {
        let temp_product = Product {
            id: product.id,
            name: product.name,
            description: product.description,
            image_url: product.image_url,
            stock: product.stock,
            price: product.price,
            category_id: product.category_id,
        };

        vec_products.push(temp_product);
    }

    Ok(HttpResponse::Ok().json(vec_products))
}