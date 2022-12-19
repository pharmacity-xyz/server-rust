use actix_web::{web, HttpRequest, HttpResponse, ResponseError};
use place_order::PlaceOrderError;
use sqlx::{Error, PgPool};
use stripe::{EventObject, EventType, Webhook, WebhookError};

use crate::{
    response::ServiceResponse,
    routes::{get_user_by_email, place_order, select_all_carts},
};

pub async fn fulfill_order(
    req: HttpRequest,
    payload: web::Bytes,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, FulFillOrderError> {
    let mut res = ServiceResponse::new(false);
    let success = handle_webhook(req, payload, &pool).await?;

    res.data = success;
    res.success = true;

    Ok(HttpResponse::Ok().json(res))
}

#[derive(Debug)]
pub enum FulFillOrderError {
    SqlxError(Error),
    WebhookError(WebhookError),
    PlaceOrder(PlaceOrderError),
}

impl std::fmt::Display for FulFillOrderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to fulfill orders.")
    }
}

impl ResponseError for FulFillOrderError {}

async fn handle_webhook(
    req: HttpRequest,
    payload: web::Bytes,
    pool: &web::Data<PgPool>,
) -> Result<bool, FulFillOrderError> {
    let payload_str = std::str::from_utf8(payload.as_ref()).unwrap();

    let stripe_signature = get_header_value(&req, "Stripe-Signature").unwrap_or_default();

    let secret = "whsec_f805a6ec3cf411e6009bf9223cd93b58d5e462707ebecf9564232d9c3903e169";

    if let Ok(event) = Webhook::construct_event(payload_str, stripe_signature, secret) {
        match event.event_type {
            EventType::CheckoutSessionCompleted => {
                if let EventObject::CheckoutSession(session) = event.data.object {
                    let success = handle_checkout_session(&pool, session).await?;
                    return Ok(success);
                }
            }
            _ => {}
        }
    }

    Ok(false)
}

fn get_header_value<'b>(req: &'b HttpRequest, key: &'b str) -> Option<&'b str> {
    req.headers().get(key)?.to_str().ok()
}

async fn handle_checkout_session(
    pool: &web::Data<PgPool>,
    session: stripe::CheckoutSession,
) -> Result<bool, FulFillOrderError> {
    let user = get_user_by_email(pool, session.customer_email.unwrap())
        .await
        .map_err(FulFillOrderError::SqlxError)?;

    let cart_item_with_products = select_all_carts(&pool, &user.user_id)
        .await
        .map_err(FulFillOrderError::SqlxError)?;

    let success = place_order(&pool, cart_item_with_products, user.user_id)
        .await
        .map_err(FulFillOrderError::PlaceOrder)?;

    Ok(success)
}
