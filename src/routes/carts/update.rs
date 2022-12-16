use actix_web::{web, HttpRequest, HttpResponse, ResponseError};
use sqlx::PgPool;

use crate::{authorization::parse_jwt, response::CartItemWithProduct, response::ServiceResponse};

pub async fn update_cart(
    req: HttpRequest,
    cart: web::Json<CartItemWithProduct>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, UpdateCartError> {
    let mut res = ServiceResponse::new(CartItemWithProduct::default());

    let (user_id, _role) = parse_jwt(&req).map_err(UpdateCartError::JwtError)?;

    sqlx::query!(
        r#"
        UPDATE cart_items 
        SET quantity = $1
        WHERE user_id = $2 AND product_id = $3
        "#,
        cart.quantity,
        user_id,
        cart.product_id,
    )
    .execute(pool.get_ref())
    .await
    .map_err(UpdateCartError::SqlxError)?;

    res.data = CartItemWithProduct {
        product_id: cart.product_id,
        product_name: cart.product_name.clone(),
        image_url: cart.image_url.clone(),
        price: cart.price.clone(),
        quantity: cart.quantity,
    };
    res.success = true;

    Ok(HttpResponse::Ok().json(res))
}

#[derive(Debug)]
pub enum UpdateCartError {
    SqlxError(sqlx::Error),
    JwtError(jsonwebtoken::errors::Error),
}

impl ResponseError for UpdateCartError {}

impl std::fmt::Display for UpdateCartError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to update carts.")
    }
}
