use actix_web::{web, HttpResponse, ResponseError};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct PostCart {
    user_id: Uuid,
    product_id: Uuid,
    quantity: i32,
}

pub async fn post_cart(
    cart: web::Json<PostCart>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, PostCartError> {
    sqlx::query!(
        r#"
        INSERT INTO cart_items (user_id, product_id, quantity)
        VALUES ($1, $2, $3)
        "#,
        cart.user_id,
        cart.product_id,
        cart.quantity,
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
        write!(f, "Failed to post carts")
    }
}
