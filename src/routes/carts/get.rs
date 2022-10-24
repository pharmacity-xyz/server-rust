use actix_web::{web, HttpResponse, ResponseError};
use sqlx::PgPool;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Cart {
    user_id: uuid::Uuid,
    product_id: uuid::Uuid,
    quantity: i32,
    id: uuid::Uuid,
}

pub async fn get_all_carts(pool: web::Data<PgPool>) -> Result<HttpResponse, GetAllCartsError> {
    let carts = sqlx::query!(
        r#"
        SELECT * FROM carts
        "#
    )
    .fetch_all(pool.get_ref())
    .await
    .map_err(GetAllCartsError)?;

    let mut vec_carts = vec![];

    for cart in carts.into_iter() {
        let temp_cart = Cart {
            user_id: cart.user_id,
            product_id: cart.product_id,
            quantity: cart.quantity,
            id: cart.id,
        };

        vec_carts.push(temp_cart);
    }

    Ok(HttpResponse::Ok().json(vec_carts))
}

#[derive(Debug)]
pub struct GetAllCartsError(sqlx::Error);

impl ResponseError for GetAllCartsError {}

impl std::fmt::Display for GetAllCartsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to get all carts.")
    }
}
