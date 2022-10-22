use crate::routes::Credentials;
use actix_web::{http::StatusCode, web, HttpResponse, ResponseError};
use secrecy::ExposeSecret;
use sqlx::PgPool;

#[derive(serde::Deserialize)]
pub struct FormData {
    username: String,
    password: Secret<String>,
}

pub async fn login(
    credential: web::Json<FormData>,
    pool: &PgPool,
) -> Result<HttpResponse, LoginError> {
    let credentials = Credentials {
        email: credential.email.clone(),
        password: credential.password.clone(),
    };

    Ok(HttpResponse::Ok().finish())
}

pub async fn validate_credentials(
    credentials: Credentials,
    pool: &PgPool,
) -> Result<uuid::Uuid, LoginError> {
    let mut user_id = None;
    let user_id: Option<_> = sqlx::query!(
        r#"SELECT id FROM users WHERE email = $1"#,
        credentials.email,
    )
    .fetch_optional(pool)
    .await?;

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
