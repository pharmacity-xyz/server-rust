use crate::{
    authentication::compute_password_hash,
    domain::{NewUser, UserEmail, UserString},
};
use actix_web::{http::StatusCode, web, HttpResponse, ResponseError};
use secrecy::Secret;
use sqlx::{PgConnection, PgPool};
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
pub async fn post_user(user: web::Json<User>, pool: web::Data<PgConnection>) -> HttpResponse {
    // let new_user = user.try_into()?;
    // println!("New User {:?}", new_user);
    sqlx::query!(
        r#"
        INSERT INTO users (id, email, password_hash, first_name, last_name, city, country, company_name, role)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#,
        uuid::Uuid::new_v4(),
        // user.id,
        user.email,
        // hashed_password.expose_secret(),
        user.password,
        user.first_name,
        user.last_name,
        user.city,
        user.country,
        user.company_name,
        "User"
    )
    .execute(pool.clone())
    .await;
    // insert_user(&pool, user).await?;
    HttpResponse::Ok().finish()
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
    let _hashed_password =
        compute_password_hash(Secret::new(user.password.clone())).expect("Failed to hash");
    sqlx::query!(
        r#"
        INSERT INTO users (id, email, password_hash, first_name, last_name, city, country, company_name, role)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#,
        uuid::Uuid::new_v4(),
        // user.id,
        user.email,
        // hashed_password.expose_secret(),
        user.password,
        user.first_name,
        user.last_name,
        user.city,
        user.country,
        user.company_name,
        "User"
    )
    .execute(pool)
    .await
    .map_err(InsertUserError)?;
    Ok(())
}

pub struct InsertUserError(sqlx::Error);

impl std::error::Error for InsertUserError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}

impl std::fmt::Debug for InsertUserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\nCaused by:\n\t{}", self, self.0)
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
