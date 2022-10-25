use actix_web::{web, HttpResponse, ResponseError};
use sqlx::PgPool;
use stripe::{
    Client, CreatePaymentLink, CreatePaymentLinkLineItems, CreatePrice, CreateProduct, Currency,
    IdOrCreate, PaymentLink, Price, Product,
};

pub async fn send_payment_link(
    _pool: web::Data<PgPool>,
) -> Result<HttpResponse, SendPaymentLinkError> {
    let secret_key = std::env::var("STRIPE_SECRET_KEY").expect("Missing STRIPE_SECRET_KEY in env");
    let client = Client::new(secret_key);

    let product = {
        let mut create_product = CreateProduct::new("T-Shirt");
        create_product.metadata = Some(
            [("async-stripe".to_string(), "true".to_string())]
                .iter()
                .cloned()
                .collect(),
        );
        Product::create(&client, create_product).await.unwrap()
    };

    let price = {
        let mut create_price = CreatePrice::new(Currency::USD);
        create_price.product = Some(IdOrCreate::Id(&product.id));
        create_price.metadata = Some(
            [("async-stripe".to_string(), "true".to_string())]
                .iter()
                .cloned()
                .collect(),
        );
        create_price.unit_amount = Some(1000);
        create_price.expand = &["product"];
        Price::create(&client, create_price).await.unwrap()
    };

    println!(
        "created a product {:?} at price {} {}",
        product.name.unwrap(),
        price.unit_amount.unwrap() / 100,
        price.currency.unwrap()
    );

    let payment_link = PaymentLink::create(
        &client,
        CreatePaymentLink::new(vec![CreatePaymentLinkLineItems {
            quantity: 3,
            price: price.id.to_string(),
            ..Default::default()
        }]),
    )
    .await
    .unwrap();

    Ok(HttpResponse::Ok().json(payment_link))
}

#[derive(Debug)]
pub struct SendPaymentLinkError(sqlx::Error);

impl ResponseError for SendPaymentLinkError {}

impl std::fmt::Display for SendPaymentLinkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to send payment link.")
    }
}
