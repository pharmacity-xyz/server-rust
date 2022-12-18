use actix_web::{web, HttpRequest, HttpResponse, ResponseError};
use sqlx::{Error, PgPool};
use stripe::{EventObject, EventType, Webhook, WebhookError};

pub async fn fulfill_order(
    req: HttpRequest,
    payload: web::Bytes,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, FulFillOrderError> {
    handle_webhook(req, payload)?;
    Ok(HttpResponse::Ok().json(""))
}

#[derive(Debug)]
pub enum FulFillOrderError {
    SqlxError(Error),
    WebhookError(WebhookError),
}

impl std::fmt::Display for FulFillOrderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to fulfill orders.")
    }
}

impl ResponseError for FulFillOrderError {}

fn handle_webhook(req: HttpRequest, payload: web::Bytes) -> Result<(), FulFillOrderError> {
    let payload_str = std::str::from_utf8(payload.as_ref()).unwrap();

    let stripe_signature = get_header_value(&req, "Stripe-Signature").unwrap_or_default();

    let secret = "whsec_f805a6ec3cf411e6009bf9223cd93b58d5e462707ebecf9564232d9c3903e169";

    if let Ok(event) = Webhook::construct_event(payload_str, stripe_signature, secret) {
        match event.event_type {
            EventType::CheckoutSessionCompleted => {
                if let EventObject::CheckoutSession(session) = event.data.object {
                    handle_checkout_session(session).map_err(FulFillOrderError::WebhookError)?;
                }
            }
            _ => {}
        }
    }

    Ok(())
}

fn get_header_value<'b>(req: &'b HttpRequest, key: &'b str) -> Option<&'b str> {
    req.headers().get(key)?.to_str().ok()
}

fn handle_checkout_session(session: stripe::CheckoutSession) -> Result<(), WebhookError> {
    println!(
        "Received checkout session completed webhook with id: {:?}",
        session.id
    );
    Ok(())
}
