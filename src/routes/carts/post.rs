use actix_web::{web, HttpResponse, ResponseError};
use sqlx::PgPool;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Cart {
    user_id: uuid::Uuid,
    product_id: uuid::Uuid,
    quantity: i32,
}

pub async fn post_cart(
    cart: web::Json<Cart>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, PostCartError> {
    sqlx::query!(
        r#"
        INSERT INTO carts (user_id, product_id, quantity, id)
        VALUES ($1, $2, $3, $4)
        "#,
        cart.user_id,
        cart.product_id,
        cart.quantity,
        uuid::Uuid::new_v4(),
    )
    .execute(pool.get_ref())
    .await
    .map_err(PostCartError)?;
    Ok(HttpResponse::Ok().finish())
}

#[derive(Debug)]
pub struct PostCartError(sqlx::Error);

impl ResponseError for PostCartError {}

impl std::fmt::Display for PostCartError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to post carts.")
    }
}
