use crate::routes::Credentials;
use actix_web::{http::StatusCode, ResponseError};
use secrecy::ExposeSecret;
use sqlx::PgPool;

pub async fn validate_credentials(
    credentials: Credentials,
    pool: &PgPool,
) -> Result<uuid::Uuid, LoginError> {
    
    let user_id: Option<_> = sqlx::query!(
        r#"
        SELECT id
        FROM users
        WHERE email = $1 
        "#,
        credentias.email
    )
    .fetch_optional(pool)
    .await
    .context("Failed to perform a query to validate auth credentials.")
    .map_err(LoginError::UnexpectedError)?;

    user_id
        .map(|row| row.id)
        .ok_or_else(|| anyhow::anyhow!("Invalid error or password"))
        .map_err(LoginError::AuthError)
}

#[derive(thiserror::Error, Debug)]
pub enum LoginError {
    #[error("Authentication failed.")]
    AuthError(#[source] anyhow::Error),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl ResponseError for LoginError {
    fn status_code(&self) -> StatusCode {
        match self {
            LoginError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            LoginError::AuthError(_) => StatusCode::UNAUTHORIZED,
        }
    }
}
