use crate::auth::{validate_credentials, Credentials};
use actix_web::{http::StatusCode, web, HttpResponse, ResponseError};
use secrecy::{ExposeSecret, Secret};
use sqlx::PgPool;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    password: Secret<String>,
}

#[tracing::instrument(skip(credential, pool), fields(email=tracing::field::Empty, user_id=tracing::field::Empty))]
pub async fn login(credential: web::Json<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    let credentials = Credentials {
        email: credential.email.clone(),
        password: credential.password.clone(),
    };
    tracing::Span::current().record("email", &tracing::field::display(&credentials.email));

    match validate_credentials(credentials, &pool).await {
        Ok(user_id) => {
            tracing::Span::current().record("email", &tracing::field::display(&user_id));
        }
        Err(_) => {
            todo!()
        }
    }

    HttpResponse::Ok().finish()
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
