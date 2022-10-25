use crate::{
    authentication::compute_password_hash,
    domain::{NewUser, UserEmail, UserString},
};
use actix_web::{http::StatusCode, web, HttpResponse, ResponseError};
use secrecy::{ExposeSecret, Secret};
use sqlx::PgPool;
use stripe::{Client, CreateCustomer, Customer, StripeError};
use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct User {
    email: String,
    password: String,
    first_name: String,
    last_name: String,
    city: String,
    country: String,
    company_name: String,
}

impl TryFrom<web::Json<User>> for NewUser {
    type Error = String;

    fn try_from(value: web::Json<User>) -> Result<Self, Self::Error> {
        let new_user = NewUser {
            id: Uuid::new_v4(),
            email: UserEmail::parse(value.email.clone())?,
            password: UserString::parse(value.password.clone())?,
            first_name: UserString::parse(value.first_name.clone())?,
            last_name: UserString::parse(value.last_name.clone())?,
            city: UserString::parse(value.city.clone())?,
            country: UserString::parse(value.country.clone())?,
            company_name: UserString::parse(value.company_name.clone())?,
            role: "User".to_string(),
        };

        Ok(new_user)
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
    user: web::Json<User>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, PostUserError> {
    // let new_user = user.try_into()?;
    insert_user(&pool, user).await?;
    Ok(HttpResponse::Ok().finish())
}

#[derive(Debug)]
pub enum PostUserError {
    ValidationError(String),
    DatabaseError(sqlx::Error),
    InsertUserError(InsertUserError),
}

impl std::fmt::Display for PostUserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to create a new user.")
    }
}

impl std::error::Error for PostUserError {}

impl ResponseError for PostUserError {
    fn status_code(&self) -> StatusCode {
        match self {
            PostUserError::ValidationError(_) => StatusCode::BAD_REQUEST,
            PostUserError::DatabaseError(_) | PostUserError::InsertUserError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}

impl From<sqlx::Error> for PostUserError {
    fn from(e: sqlx::Error) -> Self {
        Self::DatabaseError(e)
    }
}

impl From<InsertUserError> for PostUserError {
    fn from(e: InsertUserError) -> Self {
        Self::InsertUserError(e)
    }
}

impl From<String> for PostUserError {
    fn from(e: String) -> Self {
        Self::ValidationError(e)
    }
}

#[tracing::instrument(name = "Saving new user details in the database", skip(user, pool))]
pub async fn insert_user(pool: &PgPool, user: web::Json<User>) -> Result<(), InsertUserError> {
    let secret_key = std::env::var("STRIPE_SECRET_KEY").expect("Missing STRIPE_SECRET_KEY in env");
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
    .map_err(InsertUserError::CreateCustomerStripeError)?;

    let hashed_password =
        compute_password_hash(Secret::new(user.password.clone())).expect("Failed to hash");
    sqlx::query!(
        r#"
        INSERT INTO users (id, email, password_hash, first_name, last_name, city, country, company_name, role)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#,
        uuid::Uuid::new_v4(),
        user.email,
        hashed_password.expose_secret(),
        user.first_name,
        user.last_name,
        user.city,
        user.country,
        user.company_name,
        "User"
    )
    .execute(pool)
    .await
    .map_err(InsertUserError::SqlxError)?;

    Ok(())
}

pub enum InsertUserError {
    SqlxError(sqlx::Error),
    CreateCustomerStripeError(StripeError),
}

//impl std::error::Error for InsertUserError {
//    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
//        Some(&self.0)
//    }
//}

impl std::fmt::Debug for InsertUserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\nCaused by:\n\t{}", self)
    }
}

impl std::fmt::Display for InsertUserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "A database error was encountered while \
            trying to store a user"
        )
    }
}

impl ResponseError for InsertUserError {}
