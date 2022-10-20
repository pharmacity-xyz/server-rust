use crate::domain::{NewUser, UserString, UserEmail};
use actix_web::{web, HttpResponse};
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

#[tracing::instrument(
    name = "Addming a new user",
    skip(user, pool),
    fields(
        user_email = %user.email,
        user_first_name = %user.first_name
    )
)]
pub async fn post_user(user: web::Json<User>, pool: web::Data<PgPool>) -> HttpResponse {
    let new_user = NewUser {
        id: Uuid::new_v4(),
        email: UserEmail::parse(user.email.clone()).expect("Email validation failed."),
        password: UserString::parse(user.password.clone()).expect("Password validation failed."),
        first_name: UserString::parse(user.first_name.clone())
            .expect("First name validation failed."),
        last_name: UserString::parse(user.last_name.clone()).expect("Last name validation failed."),
        city: UserString::parse(user.city.clone()).expect("City validation failed."),
        country: UserString::parse(user.country.clone()).expect("Country validation failed."),
        company_name: UserString::parse(user.company_name.clone())
            .expect("Company name validation failed."),
        role: "User".to_string(),
    };

    match insert_user(&pool, &new_user).await {
        Ok(_) => HttpResponse::Ok().json(new_user),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(name = "Saving new user details in the database", skip(user, pool))]
pub async fn insert_user(pool: &PgPool, user: &NewUser) -> Result<(), sqlx::Error> {
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
