use actix_web::{web, HttpRequest, HttpResponse, ResponseError};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{authorization::parse_jwt, response::ServiceResponse};

pub async fn delete_cart(
    req: HttpRequest,
    path: web::Path<Uuid>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, DeleteCartError> {
    let mut res = ServiceResponse::new(true);
    let product_id = path.into_inner();

    let (user_id, _role) = parse_jwt(&req).map_err(DeleteCartError::JwtError)?;

    sqlx::query!(
        r#"
        DELETE FROM cart_items
        WHERE product_id = $1 AND user_id = $2
        "#,
        product_id,
        user_id,
    )
    .execute(pool.get_ref())
    .await
    .map_err(DeleteCartError::SqlxError)?;

    res.data = true;
    res.success = true;

    Ok(HttpResponse::Ok().json(res))
}

#[derive(Debug)]
pub enum DeleteCartError {
    SqlxError(sqlx::Error),
    JwtError(jsonwebtoken::errors::Error),
}

impl ResponseError for DeleteCartError {}

impl std::fmt::Display for DeleteCartError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to delete carts.")
    }
}
