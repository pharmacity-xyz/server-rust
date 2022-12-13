use actix_web::{web, HttpResponse, ResponseError};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct UpdateCart {
    user_id: Uuid,
    product_id: Uuid,
    quantity: i32,
}

pub async fn update_cart(
    cart: web::Json<UpdateCart>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, UpdateCartError> {
    sqlx::query!(
        r#"
        UPDATE cart_items 
        SET quantity = $1
        WHERE user_id = $2 AND product_id = $3
        "#,
        cart.quantity,
        cart.user_id,
        cart.product_id,
    )
    .execute(pool.get_ref())
    .await
    .map_err(UpdateCartError)?;
    Ok(HttpResponse::Ok().finish())
}

#[derive(Debug)]
pub struct UpdateCartError(sqlx::Error);

impl ResponseError for UpdateCartError {}

impl std::fmt::Display for UpdateCartError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to update carts.")
    }
}
