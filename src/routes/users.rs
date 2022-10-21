use crate::domain::{NewUser, UserEmail, UserString};
use actix_web::{web, HttpResponse, ResponseError};
use sqlx::PgPool;
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
    name = "Addming a new user",
    skip(user, pool),
    fields(
        user_email = %user.email,
        user_first_name = %user.first_name
    )
)]
pub async fn post_user(
    user: web::Json<User>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let new_user = match user.try_into() {
        Ok(user) => user,
        Err(_) => return HttpResponse::BadRequest(),
    };

    insert_user(&pool, &new_user).await?
}

#[tracing::instrument(name = "Saving new user details in the database", skip(user, pool))]
pub async fn insert_user(pool: &PgPool, user: &NewUser) -> Result<(), InsertUserError> {
    sqlx::query!(
        r#"
        INSERT INTO users (id, email, password, first_name, last_name, city, country, company_name, role)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#,
        user.id,
        user.email.as_ref(),
        user.password.as_ref(),
        user.first_name.as_ref(),
        user.last_name.as_ref(),
        user.city.as_ref(),
        user.country.as_ref(),
        user.company_name.as_ref(),
        user.role
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}

#[derive(Debug)]
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
            "A database error was encountered white \
            trying to store a user"
        )
    }
}

impl ResponseError for InsertUserError {}
