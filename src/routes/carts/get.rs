use actix_web::{web, HttpRequest, HttpResponse, ResponseError};
use sqlx::PgPool;

use crate::{
    authorization::parse_jwt,
    response::{CartItemWithProduct, ServiceResponse},
};

pub async fn get_all_carts(
    req: HttpRequest,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, GetAllCartsError> {
    let mut res = ServiceResponse::new(Vec::<CartItemWithProduct>::new());

    let (user_id, _role) = parse_jwt(&req).map_err(GetAllCartsError::JwtError)?;

    let vec_carts = select_all_carts(&pool, user_id.as_str()).await?;

    res.data = vec_carts;
    res.success = true;

    Ok(HttpResponse::Ok().json(res))
}

pub async fn select_all_carts(
    pool: &web::Data<PgPool>,
    user_id: &str,
) -> Result<Vec<CartItemWithProduct>, GetAllCartsError> {
    let carts = sqlx::query!(
        r#"
        SELECT products.product_id, product_name, image_url, price, quantity
        FROM cart_items
        JOIN products ON
        products.product_id = cart_items.product_id
        WHERE user_id = $1
        "#,
        user_id
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(GetAllCartsError::SqlxError)?;

    let mut vec_carts = vec![];

    for cart in carts.into_iter() {
        let temp_cart = CartItemWithProduct {
            product_id: cart.product_id,
            product_name: cart.product_name,
            image_url: cart.image_url,
            price: cart.price,
            quantity: cart.quantity,
        };

        vec_carts.push(temp_cart);
    }

    Ok(vec_carts)
}

#[derive(Debug)]
pub enum GetAllCartsError {
    SqlxError(sqlx::Error),
    JwtError(jsonwebtoken::errors::Error),
}

impl ResponseError for GetAllCartsError {}

impl std::fmt::Display for GetAllCartsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to get all carts.")
    }
}
