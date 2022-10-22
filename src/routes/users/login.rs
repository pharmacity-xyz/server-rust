use crate::routes::Credentials;
use actix_web::{http::StatusCode, web, HttpResponse, ResponseError};
use secrecy::ExposeSecret;
use sqlx::PgPool;

#[derive(serde::Deserialize)]
pub struct FormData {
    username: String,
    password: Secret<String>,
}

pub async fn login(credential: web::Json<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    let credentials = Credentials {
        email: credential.email.clone(),
        password: credential.password.clone(),
    };

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
