use actix_web::{web, HttpResponse, ResponseError};
use sqlx::PgPool;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Cart {
    id: uuid::Uuid,
    quantity: i32,
}

pub async fn update_cart(
    cart: web::Json<Cart>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, UpdateCartError> {
    sqlx::query!(
        r#"
        UPDATE carts 
        SET quantity = $1
        WHERE id = $2
        "#,
        cart.quantity,
        cart.id,
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