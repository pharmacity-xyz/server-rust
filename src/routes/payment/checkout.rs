use actix_web::{web, HttpRequest, HttpResponse, ResponseError};
use sqlx::PgPool;
use stripe::{
    CheckoutSession, Client, CreateCheckoutSession, CreateCheckoutSessionLineItems,
    CreateCheckoutSessionLineItemsPriceData, CreateCheckoutSessionPaymentMethodTypes,
    CreateCheckoutSessionShippingAddressCollectionAllowedCountries, Currency, StripeError,
};

use crate::{
    authorization::parse_jwt,
    response::ServiceResponse,
    routes::{get_user_email, select_all_carts, GetAllCartsError},
};

pub async fn checkout(
    req: HttpRequest,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, CheckoutError> {
    let mut res = ServiceResponse::new(String::default());
    let secret_key = std::env::var("STRIPE_SECRET_KEY").expect("Missing STRIPE_SECRET_KEY in env");
    let client = Client::new(secret_key);

    let (user_id, _role) = parse_jwt(&req).map_err(CheckoutError::JwtError)?;

    let user_email = get_user_email(&pool, user_id.as_str())
        .await
        .map_err(CheckoutError::SqlxError)?;

    let carts = select_all_carts(&pool, user_id.as_str())
        .await
        .map_err(CheckoutError::GetAllCartsError)?;

    let mut checkout_session_items = vec![];

    for cart in carts {
        let checkout_session_item = CreateCheckoutSessionLineItems {
            price_data: Some(CreateCheckoutSessionLineItemsPriceData {
                unit_amount_decimal: Some(cart.price.to_string()),
                currency: Currency::USD,
                product_data: Some(stripe::CreateCheckoutSessionLineItemsPriceDataProductData {
                    images: Some(vec![cart.image_url]),
                    name: cart.product_name,
                    ..Default::default()
                }),
                ..Default::default()
            }),
            quantity: Some(cart.quantity as u64),
            ..Default::default()
        };
        checkout_session_items.push(checkout_session_item);
    }

    let mut params = CreateCheckoutSession::new(
        "http://localhost:3000/cart",
        "http://localhost:3000/checkout/order-success",
    );
    params.customer_email = Some(user_email.as_str());
    params.shipping_address_collection =
        Some(stripe::CreateCheckoutSessionShippingAddressCollection {
            allowed_countries: vec![
                CreateCheckoutSessionShippingAddressCollectionAllowedCountries::Us,
            ],
        });
    params.payment_method_types = Some(vec![CreateCheckoutSessionPaymentMethodTypes::Card]);
    params.line_items = Some(checkout_session_items);
    params.mode = Some(stripe::CheckoutSessionMode::Payment);

    let checkout_session = CheckoutSession::create(&client, params)
        .await
        .map_err(CheckoutError::StripeError)?;

    res.data = checkout_session.url.expect("Fail to get checkout url");
    res.success = true;

    Ok(HttpResponse::Ok().json(res))
}

#[derive(Debug)]
pub enum CheckoutError {
    SqlxError(sqlx::Error),
    JwtError(jsonwebtoken::errors::Error),
    GetAllCartsError(GetAllCartsError),
    StripeError(StripeError),
}

impl ResponseError for CheckoutError {}

impl std::fmt::Display for CheckoutError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to send payment link.")
    }
}
