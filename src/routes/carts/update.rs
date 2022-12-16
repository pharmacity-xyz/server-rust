use actix_web::{web, HttpResponse, ResponseError};
use sqlx::PgPool;

use crate::{response::ServiceResponse, types::Cart};

pub async fn update_cart(
    cart: web::Json<Cart>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, UpdateCartError> {
    let mut res = ServiceResponse::new(Cart::default());

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

    res.data = cart.into();
    res.success = true;

    Ok(HttpResponse::Ok().json(res))
}

#[derive(Debug)]
pub struct UpdateCartError(sqlx::Error);

impl ResponseError for UpdateCartError {}

impl std::fmt::Display for UpdateCartError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to update carts.")
    }
}
