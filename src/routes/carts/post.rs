use actix_web::{web, HttpRequest, HttpResponse, ResponseError};
use sqlx::PgPool;

use crate::{
    authorization::parse_jwt, request::RequestPostCart, response::ServiceResponse, types::Cart,
};

pub async fn post_cart(
    req: HttpRequest,
    cart: web::Json<RequestPostCart>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, PostCartError> {
    let mut res = ServiceResponse::new(Cart::default());

    let (user_id, _role) = parse_jwt(&req).map_err(PostCartError::JwtError)?;

    sqlx::query!(
        r#"
        INSERT INTO cart_items (user_id, product_id, quantity)
        VALUES ($1, $2, $3)
        "#,
        user_id,
        cart.product_id,
        cart.quantity,
    )
    .execute(pool.get_ref())
    .await
    .map_err(PostCartError::SqlxError)?;

    res.data = Cart {
        user_id,
        product_id: cart.product_id,
        quantity: cart.quantity,
    };
    res.success = true;

    Ok(HttpResponse::Ok().json(res))
}

#[derive(Debug)]
pub enum PostCartError {
    SqlxError(sqlx::Error),
    JwtError(jsonwebtoken::errors::Error),
}

impl ResponseError for PostCartError {}

impl std::fmt::Display for PostCartError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to post carts")
    }
}
