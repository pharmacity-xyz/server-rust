use actix_web::{web, HttpResponse, ResponseError};
use sqlx::PgPool;
use stripe::{
    Client, CreateCustomer, CreatePrice, CreateProduct, Currency, Customer, IdOrCreate, Product,
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

pub async fn post_product(
    product: web::Json<RequestProduct>,
    pool: web::Data<PgPool>,
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

    sqlx::query!(
        r#"
        INSERT INTO products (id, name, description, image_url, stock, price, category_id, featured)
        VALUES ($1, $2, $3, $4, $5, $6, $7, false)
        "#,
        uuid::Uuid::new_v4(),
        product.name,
        product.description,
        product.image_url,
        product.stock,
        product.price,
        product.category_id,
    )
    .execute(pool.get_ref())
    .await
    .map_err(PostProductError)?;
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
