use actix_web::{web, HttpResponse, ResponseError};
use sqlx::PgPool;
use stripe::{
    CheckoutSession, Client, CreateCheckoutSession, CreateCheckoutSessionLineItems, CreateCustomer,
    CreatePrice, CreateProduct, Currency, Customer, Expandable, IdOrCreate, Price, Product,
};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct RequestProduct {
    name: String,
    description: String,
    image_url: String,
    stock: i32,
    price: i32,
    category_id: uuid::Uuid,
}

pub async fn post_order(
    _product: web::Json<RequestProduct>,
    _pool: web::Data<PgPool>,
) -> Result<HttpResponse, PostProductError> {
    let secret_key = std::env::var("STRIPE_SECRET_KEY").expect("Missing STRIPE_SECRET_KEY in env");
    let client = Client::new(secret_key);

    let customer = Customer::create(
        &client,
        CreateCustomer {
            name: Some("Peter"),
            email: Some("test@gmail.com"),
            description: Some(
                "A fake customer that is used to illustrate the examples in async-stripe",
            ),
            metadata: Some(
                [("async-stripe".to_string(), "true".to_string())]
                    .iter()
                    .cloned()
                    .collect(),
            ),

            ..Default::default()
        },
    )
    .await
    .unwrap();

    println!(
        "Created a customer at https://dashboard.stripe.com/test/customers/{}",
        customer.id
    );

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

    let checkout_session = {
        let mut params =
            CreateCheckoutSession::new("http://test.com/cancel", "http://test.com/success");
        params.customer = Some(customer.id);
        params.mode = Some(stripe::CheckoutSessionMode::Payment);
        params.line_items = Some(vec![CreateCheckoutSessionLineItems {
            quantity: Some(3),
            price: Some(price.id.to_string()),
            ..Default::default()
        }]);
        params.expand = &["line_items", "line_items.data.price.product"];

        CheckoutSession::create(&client, params).await.unwrap()
    };

    println!(
        "created a {} checkout session for {} {:?} for {} {} at {}",
        checkout_session.payment_status,
        checkout_session.line_items.data[0].quantity.unwrap(),
        match checkout_session.line_items.data[0]
            .price
            .as_ref()
            .unwrap()
            .product
            .as_ref()
            .unwrap()
        {
            Expandable::Object(p) => p.name.as_ref().unwrap(),
            _ => panic!("product not found"),
        },
        checkout_session.amount_subtotal.unwrap() / 100,
        checkout_session.line_items.data[0]
            .price
            .as_ref()
            .unwrap()
            .currency
            .unwrap(),
        checkout_session.url.unwrap()
    );

    Ok(HttpResponse::Ok().finish())
}

#[derive(Debug)]
pub struct PostProductError(sqlx::Error);

impl ResponseError for PostProductError {}

impl std::fmt::Display for PostProductError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to post products.")
    }
}
