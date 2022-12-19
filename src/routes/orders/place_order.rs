use actix_web::{web, ResponseError};
use bigdecimal::{BigDecimal, ToPrimitive};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{response::CartItemWithProduct, types::Order};

pub async fn place_order(
    pool: &web::Data<PgPool>,
    products: Vec<CartItemWithProduct>,
    user_id: String,
) -> Result<bool, PlaceOrderError> {
    let mut total_price: i64 = 0;

    for product in products.iter() {
        total_price += product.price.to_i64().unwrap() * product.quantity as i64;
    }

    let new_order = Order {
        order_id: Uuid::new_v4(),
        user_id: user_id.clone(),
        order_date: Utc::now(),
        shipped_date: Utc::now(),
        ship_address: "New York".to_string(),
        total_price: BigDecimal::from(total_price),
    };

    sqlx::query!(r#"INSERT INTO orders (order_id, user_id, total_price, ship_address, order_date, shipped_date)
		VALUES ($1, $2, $3, $4, $5, $6)"#, 
        new_order.order_id,
        new_order.user_id,
        new_order.total_price,
        new_order.ship_address,
        new_order.order_date,
        new_order.shipped_date
    ).execute(pool.get_ref())
        .await
        .map_err(PlaceOrderError)?;

    for product in products {
        sqlx::query!(
            r#"INSERT INTO order_items (order_id, product_id, quantity, total_price)
		VALUES ($1, $2, $3, $4)"#,
            new_order.order_id,
            product.product_id,
            product.quantity,
            product.price * BigDecimal::from(product.quantity),
        )
        .execute(pool.get_ref())
        .await
        .map_err(PlaceOrderError)?;
    }

    sqlx::query!(
        r#"DELETE FROM cart_items
		WHERE user_id = $1"#,
        user_id
    )
    .execute(pool.get_ref())
    .await
    .map_err(PlaceOrderError)?;

    Ok(true)
}

#[derive(Debug)]
pub struct PlaceOrderError(sqlx::Error);

impl ResponseError for PlaceOrderError {}

impl std::fmt::Display for PlaceOrderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to place order")
    }
}
