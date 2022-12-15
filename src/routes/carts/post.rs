use actix_web::{web, HttpResponse, ResponseError};
use sqlx::PgPool;

use crate::{response::ServiceResponse, types::Cart};

pub async fn post_cart(
    cart: web::Json<Cart>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, PostCartError> {
    let mut res = ServiceResponse::new(Cart::default());

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

    res.data = cart.into();
    res.success = true;

    Ok(HttpResponse::Ok().json(res))
}

#[derive(Debug)]
pub struct PostCartError(sqlx::Error);

impl ResponseError for PostCartError {}

impl std::fmt::Display for PostCartError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to post carts")
    }
}
