use crate::{
    authentication::compute_password_hash, request::PostUser, response::ServiceResponse,
    types::user::User,
};
use actix_web::{web, HttpResponse, ResponseError};
use secrecy::{ExposeSecret, Secret};
use sqlx::PgPool;
use stripe::{Client, CreateCustomer, Customer, StripeError};

#[derive(Debug)]
pub enum PostUserError {
    ValidationError(String),
    DatabaseError(sqlx::Error),
    StripeInsertionError(StripeError),
}

impl std::fmt::Display for PostUserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Fail to create user")
    }
}

impl ResponseError for PostUserError {}

impl From<sqlx::Error> for PostUserError {
    fn from(e: sqlx::Error) -> Self {
        Self::DatabaseError(e)
    }
}

impl From<String> for PostUserError {
    fn from(e: String) -> Self {
        Self::ValidationError(e)
    }
}

#[tracing::instrument(
    name = "Adding a new user",
    skip(user, pool),
    fields(
        user_email = %user.email,
        user_first_name = %user.first_name
    )
)]
pub async fn post_user(
    user: web::Json<PostUser>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, PostUserError> {
    let mut res = ServiceResponse::new(String::default());

    let mut new_user = match User::try_from(user) {
        Ok(u) => u,
        Err(e) => {
            res.message = e.to_string();
            return Err(e);
        }
    };

    let customer = insert_user_to_stripe(&new_user).await?;
    new_user.user_id = customer.id.to_string();

    match insert_user_to_db(&pool, &new_user).await {
        Ok(user_id) => {
            res.data = user_id;
            res.success = true;
        }
        Err(e) => res.message = e.to_string(),
    };

    Ok(HttpResponse::Ok().json(res))
}

#[tracing::instrument(name = "Saving new user details in the database", skip(user, pool))]
async fn insert_user_to_db(pool: &PgPool, user: &User) -> Result<String, PostUserError> {
    let hashed_password =
        compute_password_hash(Secret::new(user.password.inner())).expect("Failed to hash");

    let user_id = sqlx::query!(
        r#"
        INSERT INTO users (user_id, email, password, first_name, last_name, city, country, company_name, role)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9) 
        RETURNING user_id
        "#,
        user.user_id,
        user.email.inner(),
        hashed_password.expose_secret(),
        user.first_name.inner(),
        user.last_name.inner(),
        user.city.inner(),
        user.country.inner(),
        user.company_name.inner(),
        user.role,
    )
    .fetch_one(pool)
    .await
    .map_err(PostUserError::DatabaseError)?
    .user_id;

    Ok(user_id)
}

#[tracing::instrument(name = "Saving new user details in the stripe", skip(user))]
async fn insert_user_to_stripe(user: &User) -> Result<Customer, PostUserError> {
    let secret_key =
        std::env::var("STRIPE_SECRET_KEY").expect("Can not find stripe secret key in env");
    let client = Client::new(secret_key);
    let name = format!("{} {}", user.first_name, user.last_name);

    let customer = Customer::create(
        &client,
        CreateCustomer {
            name: Some(name.as_ref()),
            email: Some(user.email.as_ref()),
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
    .map_err(PostUserError::StripeInsertionError)?;

    Ok(customer)
}
